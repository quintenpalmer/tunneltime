ALTER TABLE towns ADD CONSTRAINT towns_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);
