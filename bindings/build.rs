winrt::build!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    types
        windows::application_model::data_transfer::*
        windows::data::xml::dom::*
        windows::web::syndication::*
);

fn main() {
    build();
}
