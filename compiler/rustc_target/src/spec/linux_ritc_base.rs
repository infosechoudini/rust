use crate::spec::crt_objects::{self, LinkSelfContainedDefault};
use crate::spec::TargetOptions;

pub fn opts() -> TargetOptions {
    let mut base = super::linux_base::opts();

    base.env = "ritc".into();
    base.pre_link_objects_self_contained = crt_objects::pre_ritc_self_contained();
    base.post_link_objects_self_contained = crt_objects::post_ritc_self_contained();
    base.link_self_contained = LinkSelfContainedDefault::Ritc;

    // These targets statically link libc by default
    base.crt_static_default = true;

    base
}
