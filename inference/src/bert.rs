// Bert inference using Candle
// https://github.com/huggingface/candle/blob/main/candle-examples/examples/bert/main.rs

use candle_core::{DType, Device, Tensor};
use candle_transformers::models::bert::{BertModel, Config, HiddenAct, DTYPE};
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;

struct Args {
    cpu: bool,
    tracing: bool,
    model_id: Option<String>,
    revision: Option<String>,
    use_pth: bool,
    n: usize,
    normalize_embeddings: bool,
    approximate_gelu: bool,
}

fn build_model_and_tokeniser() {
    let api = Api::new().unwrap();

    let model_id = "sentence-transformers/all-MiniLM-L6-v2".to_string();
    let revision = "refs/pr/21".to_string();

    let args = Args {
        cpu: true,
        tracing: false,
        model_id: Some(model_id.clone()),
        revision: Some(revision.clone()),
        use_pth: false,
        n: 1,
        normalize_embeddings: true,
        approximate_gelu: true,
    };

    let repo = Repo::with_revision(model_id, RepoType::Model, revision);
    let api = api.repo(repo);

    let weights_api = api.get("model.safetensors").unwrap();
    let tokeniser_filename = api.get("tokenizer.json").expect("error reaching HF hub");

    let weights = candle_core::safetensors::load(weights_api, &Device::Cpu);
    let tokeniser = Tokenizer::from_file(tokeniser_filename).expect("error reaching HF hub");
}

pub fn main() {
    //build_model_and_tokeniser();

    //let model = BertModel::load(vb, &config)?;
}
