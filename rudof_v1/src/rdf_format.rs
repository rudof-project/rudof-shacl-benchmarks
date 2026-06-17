use common::RdfFormat;
use rudof::{RDFFormat, ShaclFormat};

pub(super) fn cnv_shacl_format(value: RdfFormat) -> ShaclFormat {
    match value {
        RdfFormat::Turtle => ShaclFormat::Turtle,
        RdfFormat::NTriples => ShaclFormat::NTriples,
        RdfFormat::RdfXml => ShaclFormat::RDFXML,
        RdfFormat::TriG => ShaclFormat::TriG,
        RdfFormat::N3 => ShaclFormat::N3,
        RdfFormat::NQuads => ShaclFormat::NQuads,
        RdfFormat::JsonLd => ShaclFormat::JsonLd,
    }
}

pub(super) fn cnv_rdf_format(value: RdfFormat) -> RDFFormat {
    match value {
        RdfFormat::Turtle => RDFFormat::Turtle,
        RdfFormat::NTriples => RDFFormat::NTriples,
        RdfFormat::RdfXml => RDFFormat::RDFXML,
        RdfFormat::TriG => RDFFormat::TriG,
        RdfFormat::N3 => RDFFormat::N3,
        RdfFormat::NQuads => RDFFormat::NQuads,
        RdfFormat::JsonLd => RDFFormat::JsonLd,
    }
}
