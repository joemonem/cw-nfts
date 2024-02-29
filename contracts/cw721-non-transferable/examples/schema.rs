use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, export_schema_with_title, remove_schemas, schema_for};

use cw721::EmptyCollectionInfoExtension;
#[allow(deprecated)]
use cw721::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, CollectionInfo, ContractInfoResponse,
    Cw721ExecuteMsg, NftInfoResponse, NumTokensResponse, OperatorsResponse, OwnerOfResponse,
    TokensResponse,
};
use cw721_non_transferable::{EmptyExtension, InstantiateMsg, MinterResponse, QueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema_with_title(
        &schema_for!(InstantiateMsg<EmptyCollectionInfoExtension>),
        &out_dir,
        "InstantiateMsg",
    );
    export_schema_with_title(&schema_for!(Cw721ExecuteMsg), &out_dir, "Cw721ExecuteMsg");
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema_with_title(
        &schema_for!(AllNftInfoResponse<EmptyExtension>),
        &out_dir,
        "AllNftInfoResponse",
    );
    export_schema(&schema_for!(ApprovalResponse), &out_dir);
    export_schema(&schema_for!(ApprovalsResponse), &out_dir);
    export_schema(&schema_for!(OperatorsResponse), &out_dir);
    #[allow(deprecated)]
    export_schema(&schema_for!(ContractInfoResponse), &out_dir);
    export_schema_with_title(
        &schema_for!(CollectionInfo<EmptyCollectionInfoExtension>),
        &out_dir,
        "CollectionInfo",
    );
    export_schema(&schema_for!(MinterResponse), &out_dir);
    export_schema_with_title(
        &schema_for!(NftInfoResponse<EmptyExtension>),
        &out_dir,
        "NftInfoResponse",
    );
    export_schema(&schema_for!(NumTokensResponse), &out_dir);
    export_schema(&schema_for!(OwnerOfResponse), &out_dir);
    export_schema(&schema_for!(TokensResponse), &out_dir);
}
