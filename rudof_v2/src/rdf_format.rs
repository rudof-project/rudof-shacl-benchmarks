use common::RdfFormat;
use rudof::formats::{DataFormat, ShaclFormat};

pub(super) fn cnv_shacl_format(value: RdfFormat) -> ShaclFormat {
    match value {
        RdfFormat::Turtle => ShaclFormat::Turtle,
        RdfFormat::NTriples => ShaclFormat::NTriples,
        RdfFormat::RdfXml => ShaclFormat::RdfXml,
        RdfFormat::TriG => ShaclFormat::TriG,
        RdfFormat::N3 => ShaclFormat::N3,
        RdfFormat::NQuads => ShaclFormat::NQuads,
        RdfFormat::JsonLd => ShaclFormat::JsonLd,
    }
}

pub(super) fn cnv_rdf_format(value: RdfFormat) -> DataFormat {
    match value {
        RdfFormat::Turtle => DataFormat::Turtle,
        RdfFormat::NTriples => DataFormat::NTriples,
        RdfFormat::RdfXml => DataFormat::RdfXml,
        RdfFormat::TriG => DataFormat::TriG,
        RdfFormat::N3 => DataFormat::N3,
        RdfFormat::NQuads => DataFormat::NQuads,
        RdfFormat::JsonLd => DataFormat::JsonLd,
    }
}
