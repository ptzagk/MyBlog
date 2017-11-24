use sapper::{ SapperModule, SapperRouter, Response, Request, Result as SapperResult, Error as SapperError };
use sapper_std::{ PathParams, JsonParams, SessionVal };
use serde_json;

use super::super::{ NewTag, RelationTag, Tags, Relations, TagCount, Postgresql, Redis, admin_verification_cookie };

pub struct Tag;

impl Tag {
    fn create_tag(req: &mut Request) -> SapperResult<Response> {
        let body: NewTag = get_json_params!(req);
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();

        if body.insert(&pg_pool) {
            res_json!(json!({"status": true}))
        } else {
            res_json!(json!({"status": false}))
        }
    }

    fn create_relation(req: &mut Request) -> SapperResult<Response> {
        let body: RelationTag = get_json_params!(req);
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();

        if body.insert(&pg_pool) {
            res_json!(json!({"status": true}))
        } else {
            res_json!(json!({"status": false}))
        }
    }

    fn delete_relation(req: &mut Request) -> SapperResult<Response> {
        let body: Relations = get_json_params!(req);
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();

        if body.delete_relation(&pg_pool) {
            res_json!(json!({"status": true}))
        } else {
            res_json!(json!({"status": false}))
        }
    }

    fn delete_tag(req: &mut Request) -> SapperResult<Response> {
        let params = get_path_params!(req);
        let id: i32 = t_param!(params, "id").clone().parse().unwrap();
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        let res = match Tags::delete_tag(&pg_pool, id) {
            Ok(num_deleted) => {
                json!({
                    "status": true,
                    "num_deleted": num_deleted
                    })
            },
            Err(err) => {
                json!({
                    "status": false,
                    "error": err
                    })
            }
        };
        res_json!(res)
    }

    fn view_tag(req: &mut Request) -> SapperResult<Response> {
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        let res = match TagCount::view_tag_count(&pg_pool) {
            Ok(data) => {
                json!({
                    "status": true,
                    "data": data
                })
            }
            Err(err) => {
                json!({
                    "status": false,
                    "error": err
                })
            }
        };
        res_json!(res)
    }

    fn edit_tag(req: &mut Request) -> SapperResult<Response> {
        let body: Tags = get_json_params!(req);
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        let res = match body.edit_tag(&pg_pool) {
            Ok(num_update) => {
                json!({
                    "status": true,
                    "num_update": num_update
                })
            }
            Err(err) => {
                json!({
                    "status": false,
                    "error": format!("{}", err)
                })
            }
        };
        res_json!(res)
    }
}

impl SapperModule for Tag {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        let cookie = req.ext().get::<SessionVal>();
        let redis_pool = req.ext().get::<Redis>().unwrap();
        match admin_verification_cookie(cookie, redis_pool) {
            true => { Ok(()) }
            false => {
                let res = json!({
                    "status": false,
                    "error": String::from("Verification error")
                });
                Err(SapperError::CustomJson(res.to_string()))
            }
        }
    }

    fn after(&self, _req: &Request, _res: &mut Response) -> SapperResult<()> {
        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        // http get :8888/tag/view
        router.get("/tag/view", Tag::view_tag);

        // http post :8888/tag/new tag="Rust"
        router.post("/tag/new", Tag::create_tag);

        // http post :8888/tag/delete/3
        router.post("/tag/delete/:id", Tag::delete_tag);

        // http post :8888/tag/edit id:=2 tag="Linux&&Rust"
        router.post("/tag/edit", Tag::edit_tag);

        // http post :8888/relation/new tag="Python" article_id:=1 tag_id:=
        router.post("/relation/new", Tag::create_relation);

        // http post :8888/relation/delete tag_id:=2  article_id:=1
        router.post("/relation/delete", Tag::delete_relation);
        Ok(())
    }
}
