use indexmap::*;
use openapiv3::*;

pub fn petstore() -> OpenAPI {
    OpenAPI {
        openapi: "3.0.0".to_owned(),
        info: Info {
            title: "Swagger Petstore".to_owned(),
            license: Some(License {
                name: "MIT".to_owned(),
                url: None,
            }),
            version: "1.0.0".to_owned(),
            ..Default::default()
        },
        servers: vec![Server {
            url: "http://petstore.swagger.io/v1".to_owned(),
            ..Default::default()
        }],
        components: Some(Components {
            schemas: indexmap! {
                "Cat".to_owned() => ReferenceOr::Item(Schema {
                    schema_data: SchemaData {
                        description: Some("A representation of a cat".to_owned()),
                        ..Default::default()
                    },
                    schema_kind: SchemaKind::AllOf { all_of: vec![
                        ReferenceOr::ref_("#/components/schemas/Pet"),
                        ReferenceOr::Item(Schema {
                            schema_data: Default::default(),
                            schema_kind: SchemaKind::Type(Type::Object(ObjectType {
                                properties: indexmap!{
                                    "huntingSkill".to_owned() => ReferenceOr::boxed_item(Schema {
                                        schema_data: SchemaData {
                                            description: Some("The measured skill for hunting".to_owned()),
                                            ..Default::default()
                                        },
                                        schema_kind: SchemaKind::Type(Type::String(StringType {
                                            enumeration: vec![
                                                "clueless".to_owned(),
                                                "lazy".to_owned(),
                                                "adventurous".to_owned(),
                                                "aggressive".to_owned(),
                                            ],
                                            ..Default::default()
                                        })),
                                    }),
                                },
                                required: vec!["huntingSkill".to_owned()],
                                ..Default::default()
                            })),
                        }),
                    ]},
                }),

                "Dog".to_owned() => ReferenceOr::Item(Schema {
                    schema_data: SchemaData {
                        description: Some("A representation of a dog".to_owned()),
                        ..Default::default()
                    },
                    schema_kind: SchemaKind::AllOf { all_of: vec![
                        ReferenceOr::ref_("#/components/schemas/Pet"),
                        ReferenceOr::Item(Schema {
                            schema_data: Default::default(),
                            schema_kind: SchemaKind::Type(Type::Object(ObjectType {
                                properties: indexmap!{
                                    "packSize".to_owned() => ReferenceOr::boxed_item(Schema {
                                        schema_data: SchemaData {
                                            description: Some("the size of the pack the dog is from".to_owned()),
                                            ..Default::default()
                                        },
                                        schema_kind: SchemaKind::Type(Type::Integer(IntegerType {
                                            format: VariantOrUnknownOrEmpty::Item(IntegerFormat::Int32),
                                            minimum: Some(0),
                                            ..Default::default()
                                        })),
                                    }),
                                },
                                required: vec!["packSize".to_owned()],
                                ..Default::default()
                            })),
                        }),
                    ]},
                }),

                "Pet".to_owned() => ReferenceOr::Item(Schema {
                    schema_data: SchemaData {
                        discriminator: Some(Discriminator {
                            property_name: "petType".to_owned(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    schema_kind: SchemaKind::Type(Type::Object(ObjectType {
                        properties: indexmap!{
                            "name".to_owned() => ReferenceOr::boxed_item(Schema {
                                schema_data: Default::default(),
                                schema_kind: SchemaKind::Type(Type::String(Default::default())),
                            }),
                            "petType".to_owned() => ReferenceOr::boxed_item(Schema {
                                schema_data: Default::default(),
                                schema_kind: SchemaKind::Type(Type::String(Default::default())),
                            }),
                        },
                        required: vec!["name".to_owned(), "petType".to_owned()],
                        ..Default::default()
                    })),
                }),
            },
            ..Default::default()
        }),
        ..Default::default()
    }
}
