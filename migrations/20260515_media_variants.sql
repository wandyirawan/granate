-- 20260515_media_variants.sql
-- Media variants (thumb, catalog, full) for uploaded images

CREATE TABLE IF NOT EXISTS media_variants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    media_id UUID NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    variant TEXT NOT NULL CHECK (variant IN ('thumb', 'catalog', 'full')),
    storage_key TEXT NOT NULL UNIQUE,
    width INT NOT NULL,
    height INT NOT NULL,
    size_bytes BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(media_id, variant)
);

CREATE INDEX idx_media_variants_media ON media_variants(media_id);

-- Product pages: CMS-rich content linked to Salak products
CREATE TABLE IF NOT EXISTS product_pages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salak_product_id INT NOT NULL,
    long_description TEXT DEFAULT '',
    gallery_media_ids UUID[] DEFAULT '{}',
    specs JSONB DEFAULT '{}',
    seo_title TEXT DEFAULT '',
    seo_description TEXT DEFAULT '',
    seo_keywords TEXT DEFAULT '',
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'archived')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_product_pages_salak ON product_pages(salak_product_id);
CREATE INDEX idx_product_pages_status ON product_pages(status);

-- Product-Blog relation: link blog entries to Salak products
CREATE TABLE IF NOT EXISTS product_blog_links (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entry_id UUID NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
    salak_product_id INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(entry_id, salak_product_id)
);

CREATE INDEX idx_product_blog_links_entry ON product_blog_links(entry_id);
CREATE INDEX idx_product_blog_links_salak ON product_blog_links(salak_product_id);

-- Trigger for product_pages updated_at
CREATE TRIGGER update_product_pages_updated_at BEFORE UPDATE ON product_pages
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
