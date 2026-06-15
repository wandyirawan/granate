-- Seed dummy data: Toko Busana Muslim
-- Run: podman exec -i jambu-postgres psql -U postgres -d granate < seeds/seed_busana_muslim.sql

-- ============================================
-- PRIBA — Baju Koko
-- ============================================
INSERT INTO parent_products (id, name, slug, description, option_types, status)
VALUES (uuid_generate_v4(), 'Baju Koko Premium', 'baju-koko-premium', 'Baju koko berbahan katun premium, adem dan nyaman dipakai sholat maupun acara formal.', '["color", "size"]', 'published');

WITH parent AS (SELECT id FROM parent_products WHERE slug = 'baju-koko-premium')
INSERT INTO product_variants (parent_id, salak_sku, option_values, is_default, sort_order)
VALUES
  ((SELECT id FROM parent), 'KOKO-WHT-M', '{"color": "Putih", "size": "M"}', true, 0),
  ((SELECT id FROM parent), 'KOKO-WHT-L', '{"color": "Putih", "size": "L"}', false, 1),
  ((SELECT id FROM parent), 'KOKO-WHT-XL', '{"color": "Putih", "size": "XL"}', false, 2),
  ((SELECT id FROM parent), 'KOKO-BLK-M', '{"color": "Hitam", "size": "M"}', false, 3),
  ((SELECT id FROM parent), 'KOKO-BLK-L', '{"color": "Hitam", "size": "L"}', false, 4),
  ((SELECT id FROM parent), 'KOKO-NAV-M', '{"color": "Navy", "size": "M"}', false, 5),
  ((SELECT id FROM parent), 'KOKO-NAV-L', '{"color": "Navy", "size": "L"}', false, 6);

-- ============================================
-- PRIBA — Kemeja Muslim
-- ============================================
INSERT INTO parent_products (id, name, slug, description, option_types, status)
VALUES (uuid_generate_v4(), 'Kemeja Muslim Pria', 'kemeja-muslim-pria', 'Kemeja lengan panjang motif subtle, cocok untuk kerja maupun santai.', '["color", "size"]', 'published');

WITH parent AS (SELECT id FROM parent_products WHERE slug = 'kemeja-muslim-pria')
INSERT INTO product_variants (parent_id, salak_sku, option_values, is_default, sort_order)
VALUES
  ((SELECT id FROM parent), 'KEMEJA-BLU-M', '{"color": "Biru", "size": "M"}', true, 0),
  ((SELECT id FROM parent), 'KEMEJA-BLU-L', '{"color": "Biru", "size": "L"}', false, 1),
  ((SELECT id FROM parent), 'KEMEJA-BLU-XL', '{"color": "Biru", "size": "XL"}', false, 2),
  ((SELECT id FROM parent), 'KEMEJA-GRY-M', '{"color": "Abu-abu", "size": "M"}', false, 3),
  ((SELECT id FROM parent), 'KEMEJA-GRY-L', '{"color": "Abu-abu", "size": "L"}', false, 4),
  ((SELECT id FROM parent), 'KEMEJA-WHT-M', '{"color": "Putih", "size": "M"}', false, 5),
  ((SELECT id FROM parent), 'KEMEJA-WHT-L', '{"color": "Putih", "size": "L"}', false, 6);

-- ============================================
-- PRIBA — Celana Panjang
-- ============================================
INSERT INTO parent_products (id, name, slug, description, option_types, status)
VALUES (uuid_generate_v4(), 'Celana Panjang Muslim', 'celana-panjang-muslim', 'Celana bahan cotton twill, cutting lurus, nyaman buat ibadah dan harian.', '["color", "size"]', 'published');

WITH parent AS (SELECT id FROM parent_products WHERE slug = 'celana-panjang-muslim')
INSERT INTO product_variants (parent_id, salak_sku, option_values, is_default, sort_order)
VALUES
  ((SELECT id FROM parent), 'CEL-BLK-M', '{"color": "Hitam", "size": "M"}', true, 0),
  ((SELECT id FROM parent), 'CEL-BLK-L', '{"color": "Hitam", "size": "L"}', false, 1),
  ((SELECT id FROM parent), 'CEL-BLK-XL', '{"color": "Hitam", "size": "XL"}', false, 2),
  ((SELECT id FROM parent), 'CEL-NAV-M', '{"color": "Navy", "size": "M"}', false, 3),
  ((SELECT id FROM parent), 'CEL-NAV-L', '{"color": "Navy", "size": "L"}', false, 4),
  ((SELECT id FROM parent), 'CEL-GRY-M', '{"color": "Abu-abu", "size": "M"}', false, 5),
  ((SELECT id FROM parent), 'CEL-GRY-L', '{"color": "Abu-abu", "size": "L"}', false, 6);

-- ============================================
-- PRIBA — Sarung
-- ============================================
INSERT INTO parent_products (id, name, slug, description, option_types, status)
VALUES (uuid_generate_v4(), 'Sarung Tenun', 'sarung-tenun', 'Sarung tenun asli, nyaman dipakai sholat berjamaah di masjid.', '["color"]', 'published');

WITH parent AS (SELECT id FROM parent_products WHERE slug = 'sarung-tenun')
INSERT INTO product_variants (parent_id, salak_sku, option_values, is_default, sort_order)
VALUES
  ((SELECT id FROM parent), 'SRG-BRN', '{"color": "Coklat"}', true, 0),
  ((SELECT id FROM parent), 'SRG-DGN', '{"color": "Hijau Tua"}', false, 1),
  ((SELECT id FROM parent), 'SRG-BLK', '{"color": "Hitam"}', false, 2);

-- ============================================
-- PRIBA — Peci
-- ============================================
INSERT INTO parent_products (id, name, slug, description, option_types, status)
VALUES (uuid_generate_v4(), 'Peci Rajut', 'peci-rajut', 'Peci rajut premium, border tinggi, tersedia berbagai warna.', '["color"]', 'published');

WITH parent AS (SELECT id FROM parent_products WHERE slug = 'peci-rajut')
INSERT INTO product_variants (parent_id, salak_sku, option_values, is_default, sort_order)
VALUES
  ((SELECT id FROM parent), 'PECI-BLK', '{"color": "Hitam"}', true, 0),
  ((SELECT id FROM parent), 'PECI-WHT', '{"color": "Putih"}', false, 1),
  ((SELECT id FROM parent), 'PECI-NAV', '{"color": "Navy"}', false, 2);

-- ============================================
-- WANITA — Gamis
-- ============================================
INSERT INTO parent_products (id, name, slug, description, option_types, status)
VALUES (uuid_generate_v4(), 'Gamis Syari Premium', 'gamis-syari-premium', 'Gamis bahan katun rayon, flowy dan syari. Cocok untuk daily maupun acara.', '["color", "size"]', 'published');

WITH parent AS (SELECT id FROM parent_products WHERE slug = 'gamis-syari-premium')
INSERT INTO product_variants (parent_id, salak_sku, option_values, is_default, sort_order)
VALUES
  ((SELECT id FROM parent), 'GAMIS-BLK-M', '{"color": "Hitam", "size": "M"}', true, 0),
  ((SELECT id FROM parent), 'GAMIS-BLK-L', '{"color": "Hitam", "size": "L"}', false, 1),
  ((SELECT id FROM parent), 'GAMIS-BLK-XL', '{"color": "Hitam", "size": "XL"}', false, 2),
  ((SELECT id FROM parent), 'GAMIS-NAV-M', '{"color": "Navy", "size": "M"}', false, 3),
  ((SELECT id FROM parent), 'GAMIS-NAV-L', '{"color": "Navy", "size": "L"}', false, 4),
  ((SELECT id FROM parent), 'GAMIS-BRG-M', '{"color": "Burgundy", "size": "M"}', false, 5),
  ((SELECT id FROM parent), 'GAMIS-BRG-L', '{"color": "Burgundy", "size": "L"}', false, 6),
  ((SELECT id FROM parent), 'GAMIS-DST-M', '{"color": "Dusty Pink", "size": "M"}', false, 7);

-- ============================================
-- WANITA — Hijab Instan
-- ============================================
INSERT INTO parent_products (id, name, slug, description, option_types, status)
VALUES (uuid_generate_v4(), 'Hijab Instan Voal', 'hijab-instan-voal', 'Hijab instan bahan voal premium, udah ada ciput, tinggal pakai aja.', '["color"]', 'published');

WITH parent AS (SELECT id FROM parent_products WHERE slug = 'hijab-instan-voal')
INSERT INTO product_variants (parent_id, salak_sku, option_values, is_default, sort_order)
VALUES
  ((SELECT id FROM parent), 'HIJAB-BGE', '{"color": "Beige"}', true, 0),
  ((SELECT id FROM parent), 'HIJAB-BLK', '{"color": "Hitam"}', false, 1),
  ((SELECT id FROM parent), 'HIJAB-DST', '{"color": "Dusty Pink"}', false, 2),
  ((SELECT id FROM parent), 'HIJAB-MOC', '{"color": "Mocha"}', false, 3),
  ((SELECT id FROM parent), 'HIJAB-SAG', '{"color": "Sage Green"}', false, 4);

-- ============================================
-- WANITA — Tunik
-- ============================================
INSERT INTO parent_products (id, name, slug, description, option_types, status)
VALUES (uuid_generate_v4(), 'Tunik Casual', 'tunik-casual', 'Tunik lengan panjang bahan katun, cutting loose, nyaman buat daily wear.', '["color", "size"]', 'published');

WITH parent AS (SELECT id FROM parent_products WHERE slug = 'tunik-casual')
INSERT INTO product_variants (parent_id, salak_sku, option_values, is_default, sort_order)
VALUES
  ((SELECT id FROM parent), 'TUNIK-WHT-M', '{"color": "Putih", "size": "M"}', true, 0),
  ((SELECT id FROM parent), 'TUNIK-WHT-L', '{"color": "Putih", "size": "L"}', false, 1),
  ((SELECT id FROM parent), 'TUNIK-WHT-XL', '{"color": "Putih", "size": "XL"}', false, 2),
  ((SELECT id FROM parent), 'TUNIK-SAG-M', '{"color": "Sage", "size": "M"}', false, 3),
  ((SELECT id FROM parent), 'TUNIK-SAG-L', '{"color": "Sage", "size": "L"}', false, 4),
  ((SELECT id FROM parent), 'TUNIK-BLK-M', '{"color": "Hitam", "size": "M"}', false, 5),
  ((SELECT id FROM parent), 'TUNIK-BLK-L', '{"color": "Hitam", "size": "L"}', false, 6);

-- ============================================
-- WANITA — Rok Muslim
-- ============================================
INSERT INTO parent_products (id, name, slug, description, option_types, status)
VALUES (uuid_generate_v4(), 'Rok Muslim A-Line', 'rok-muslim-a-line', 'Rok panjang cutting A-line, ada kantong samping. Bahan katun stretch nyaman bergerak.', '["color", "size"]', 'published');

WITH parent AS (SELECT id FROM parent_products WHERE slug = 'rok-muslim-a-line')
INSERT INTO product_variants (parent_id, salak_sku, option_values, is_default, sort_order)
VALUES
  ((SELECT id FROM parent), 'ROK-BLK-M', '{"color": "Hitam", "size": "M"}', true, 0),
  ((SELECT id FROM parent), 'ROK-BLK-L', '{"color": "Hitam", "size": "L"}', false, 1),
  ((SELECT id FROM parent), 'ROK-BLK-XL', '{"color": "Hitam", "size": "XL"}', false, 2),
  ((SELECT id FROM parent), 'ROK-NAV-M', '{"color": "Navy", "size": "M"}', false, 3),
  ((SELECT id FROM parent), 'ROK-NAV-L', '{"color": "Navy", "size": "L"}', false, 4),
  ((SELECT id FROM parent), 'ROK-BRN-M', '{"color": "Coklat", "size": "M"}', false, 5);

-- ============================================
-- WANITA — Cardigan Muslim
-- ============================================
INSERT INTO parent_products (id, name, slug, description, option_types, status)
VALUES (uuid_generate_v4(), 'Cardigan Outer Muslim', 'cardigan-outer-muslim', 'Outer cardigan panjang bahan rayon, flowy dan anggun. Cocok untuk layering.', '["color", "size"]', 'published');

WITH parent AS (SELECT id FROM parent_products WHERE slug = 'cardigan-outer-muslim')
INSERT INTO product_variants (parent_id, salak_sku, option_values, is_default, sort_order)
VALUES
  ((SELECT id FROM parent), 'CARD-BGE-M', '{"color": "Beige", "size": "M"}', true, 0),
  ((SELECT id FROM parent), 'CARD-BGE-L', '{"color": "Beige", "size": "L"}', false, 1),
  ((SELECT id FROM parent), 'CARD-BLK-M', '{"color": "Hitam", "size": "M"}', false, 2),
  ((SELECT id FROM parent), 'CARD-BLK-L', '{"color": "Hitam", "size": "L"}', false, 3),
  ((SELECT id FROM parent), 'CARD-GRY-M', '{"color": "Abu-abu", "size": "M"}', false, 4);

-- Verify counts
SELECT 'parent_products' AS tabel, count(*) FROM parent_products
UNION ALL
SELECT 'product_variants', count(*) FROM product_variants;
