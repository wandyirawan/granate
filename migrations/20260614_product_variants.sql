-- Parent products: groups variants together
CREATE TABLE IF NOT EXISTS parent_products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    description TEXT DEFAULT '',
    thumbnail_media_id UUID REFERENCES media(id),
    option_types JSONB NOT NULL DEFAULT '[]',  -- ["color", "size"]
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'archived')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_parent_products_slug ON parent_products(slug);

-- Product variants: links parent to Salak SKUs
CREATE TABLE IF NOT EXISTS product_variants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    parent_id UUID NOT NULL REFERENCES parent_products(id) ON DELETE CASCADE,
    salak_sku TEXT NOT NULL,
    option_values JSONB NOT NULL DEFAULT '{}',  -- {"color": "Red", "size": "M"}
    is_default BOOLEAN DEFAULT false,
    sort_order INT DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_product_variants_parent ON product_variants(parent_id);
CREATE UNIQUE INDEX idx_variant_parent_sku ON product_variants(parent_id, salak_sku);

-- Trigger
CREATE TRIGGER update_parent_products_updated_at BEFORE UPDATE ON parent_products
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();