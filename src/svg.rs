use svgcleaner::{CleaningOptions, StyleJoinMode};
use svgdom::{ParseOptions, WriteOptions};

pub fn gen_parse_options() -> ParseOptions {
  ParseOptions {
    parse_comments: true,
    parse_declarations: true,
    parse_unknown_elements: true,
    parse_unknown_attributes: true,
    parse_px_unit: false,
    skip_unresolved_classes: true,
    skip_invalid_attributes: false,
    skip_invalid_css: false,
    skip_paint_fallback: false,
  }
}

pub fn gen_clean_options() -> CleaningOptions {
  CleaningOptions {
    remove_unused_defs: true,
    convert_shapes: true,
    remove_title: true,
    remove_desc: true,
    remove_metadata: true,
    remove_dupl_linear_gradients: true,
    remove_dupl_radial_gradients: true,
    remove_dupl_fe_gaussian_blur: true,
    ungroup_groups: true,
    ungroup_defs: true,
    group_by_style: true,
    merge_gradients: true,
    regroup_gradient_stops: true,
    remove_invalid_stops: true,
    remove_invisible_elements: true,
    resolve_use: true,

    remove_version: true,
    remove_unreferenced_ids: true,
    trim_ids: true,
    remove_text_attributes: true,
    remove_unused_coordinates: true,
    remove_default_attributes: true,
    remove_xmlns_xlink_attribute: true,
    remove_needless_attributes: true,
    remove_gradient_attributes: true,
    join_style_attributes: StyleJoinMode::None,
    apply_transform_to_gradients: true,
    apply_transform_to_shapes: true,

    paths_to_relative: true,
    remove_unused_segments: true,
    convert_segments: true,
    apply_transform_to_paths: true,

    coordinates_precision: 5,
    properties_precision: 6,
    paths_coordinates_precision: 8,
    transforms_precision: 8,
  }
}

pub fn gen_write_options() -> WriteOptions {
  use svgdom::{AttributesOrder, Indent, ListSeparator};

  // Initial options should be opposite to default ones.
  WriteOptions {
    indent: Indent::None,
    attributes_indent: Indent::None,
    use_single_quote: false,
    trim_hex_colors: false,
    write_hidden_attributes: false,
    remove_leading_zero: true,
    use_compact_path_notation: false,
    join_arc_to_flags: false,
    remove_duplicated_path_commands: true,
    use_implicit_lineto_commands: false,
    simplify_transform_matrices: false,
    list_separator: ListSeparator::Space,
    attributes_order: AttributesOrder::Alphabetical,
  }
}
