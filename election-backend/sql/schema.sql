-- ============================================================
--  DISTRICTS / REGIONS
-- ============================================================

CREATE TABLE district (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    region TEXT
);

-- ============================================================
--  POLITICAL PARTIES
-- ============================================================

CREATE TABLE party (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    abbreviation TEXT UNIQUE,
    founded_year INT
);

-- ============================================================
--  ELECTIONS
-- ============================================================

CREATE TABLE election (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    date DATE NOT NULL,
    district_id INT NOT NULL REFERENCES district(id) ON DELETE CASCADE,
    description TEXT
);

CREATE INDEX idx_election_date ON election(date);
CREATE INDEX idx_election_district ON election(district_id);

-- ============================================================
--  VOTERS
-- ============================================================

CREATE TABLE voter (
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    dob DATE NOT NULL,
    national_id TEXT UNIQUE NOT NULL,
    district_id INT NOT NULL REFERENCES district(id) ON DELETE CASCADE,
    registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);

CREATE INDEX idx_voter_district ON voter(district_id);

-- ============================================================
--  CANDIDATES
-- ============================================================

CREATE TABLE candidate (
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    national_id TEXT UNIQUE NOT NULL,
    party_id INT NOT NULL REFERENCES party(id) ON DELETE CASCADE,
    district_id INT NOT NULL REFERENCES district(id) ON DELETE CASCADE,
    election_id INT NOT NULL REFERENCES election(id) ON DELETE CASCADE,
    registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT candidate_unique_election UNIQUE (national_id, election_id)
);

CREATE INDEX idx_candidate_district ON candidate(district_id);
CREATE INDEX idx_candidate_election ON candidate(election_id);

-- ============================================================
--  VOTES
-- ============================================================

CREATE TABLE vote (
    id SERIAL PRIMARY KEY,
    voter_id INT NOT NULL REFERENCES voter(id) ON DELETE CASCADE,
    candidate_id INT NOT NULL REFERENCES candidate(id) ON DELETE CASCADE,
    election_id INT NOT NULL REFERENCES election(id) ON DELETE CASCADE,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_vote_per_election UNIQUE (voter_id, election_id)
);

CREATE INDEX idx_vote_voter ON vote(voter_id);
CREATE INDEX idx_vote_candidate ON vote(candidate_id);
CREATE INDEX idx_vote_election ON vote(election_id);

-- ============================================================
--  ENFORCEMENT TRIGGERS (DATA INTEGRITY)
-- ============================================================

-- 1 Ensure voter can only vote in their own district
CREATE OR REPLACE FUNCTION enforce_voter_district_match()
RETURNS TRIGGER AS $$
BEGIN
    IF (
        SELECT v.district_id FROM voter v WHERE v.id = NEW.voter_id
    ) != (
        SELECT e.district_id FROM election e WHERE e.id = NEW.election_id
    ) THEN
        RAISE EXCEPTION 'Voter district does not match election district.';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_voter_district_match
BEFORE INSERT ON vote
FOR EACH ROW
EXECUTE FUNCTION enforce_voter_district_match();


-- 2 Ensure candidate and vote election/district match
CREATE OR REPLACE FUNCTION enforce_candidate_election_match()
RETURNS TRIGGER AS $$
BEGIN
    IF (
        SELECT c.election_id FROM candidate c WHERE c.id = NEW.candidate_id
    ) != NEW.election_id THEN
        RAISE EXCEPTION 'Candidate and vote election mismatch.';
    END IF;

    IF (
        SELECT c.district_id FROM candidate c WHERE c.id = NEW.candidate_id
    ) != (
        SELECT v.district_id FROM voter v WHERE v.id = NEW.voter_id
    ) THEN
        RAISE EXCEPTION 'Candidate and voter belong to different districts.';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_candidate_election_match
BEFORE INSERT ON vote
FOR EACH ROW
EXECUTE FUNCTION enforce_candidate_election_match();

-- ============================================================
--  AUDIT LOGGING SYSTEM
-- ============================================================

CREATE TABLE audit_log (
    id SERIAL PRIMARY KEY,
    table_name TEXT NOT NULL,
    operation TEXT NOT NULL CHECK (operation IN ('INSERT', 'UPDATE', 'DELETE')),
    record_id INT,
    old_data JSONB,
    new_data JSONB,
    changed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    changed_by TEXT DEFAULT current_user
);

-- Universal audit trigger function
CREATE OR REPLACE FUNCTION audit_trigger_fn()
RETURNS TRIGGER AS $$
DECLARE
    new_data_json JSONB;
    old_data_json JSONB;
BEGIN
    IF (TG_OP = 'INSERT') THEN
        new_data_json := to_jsonb(NEW);
        INSERT INTO audit_log(table_name, operation, record_id, new_data)
        VALUES (TG_TABLE_NAME, 'INSERT', NEW.id, new_data_json);
        RETURN NEW;

    ELSIF (TG_OP = 'UPDATE') THEN
        new_data_json := to_jsonb(NEW);
        old_data_json := to_jsonb(OLD);
        INSERT INTO audit_log(table_name, operation, record_id, old_data, new_data)
        VALUES (TG_TABLE_NAME, 'UPDATE', NEW.id, old_data_json, new_data_json);
        RETURN NEW;

    ELSIF (TG_OP = 'DELETE') THEN
        old_data_json := to_jsonb(OLD);
        INSERT INTO audit_log(table_name, operation, record_id, old_data)
        VALUES (TG_TABLE_NAME, 'DELETE', OLD.id, old_data_json);
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Attach audit triggers to key tables
CREATE TRIGGER audit_voter_changes
AFTER INSERT OR UPDATE OR DELETE ON voter
FOR EACH ROW EXECUTE FUNCTION audit_trigger_fn();

CREATE TRIGGER audit_candidate_changes
AFTER INSERT OR UPDATE OR DELETE ON candidate
FOR EACH ROW EXECUTE FUNCTION audit_trigger_fn();

CREATE TRIGGER audit_vote_changes
AFTER INSERT OR UPDATE OR DELETE ON vote
FOR EACH ROW EXECUTE FUNCTION audit_trigger_fn();
