ALTER TABLE towns ADD CONSTRAINT towns_gold_gt_zero CHECK (gold >= 0);
