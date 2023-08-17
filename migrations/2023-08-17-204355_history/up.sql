ALTER TABLE history ADD COLUMN url VARCHAR(255);
ALTER TABLE history ADD COLUMN description VARCHAR(255);
ALTER TABLE history ADD COLUMN loginLastModified TEXT; --copied from logins table
ALTER TABLE history ADD COLUMN mode TEXT;
ALTER TABLE history ADD COLUMN lastModified TEXT;

