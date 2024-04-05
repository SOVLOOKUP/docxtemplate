use j4rs::{Jvm, MavenArtifact};
pub fn deps(jvm: &Jvm) {
  let dbx_artifact = MavenArtifact::from("com.github.SOVLOOKUP:docx-template:1.1.4");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("com.deepoove:poi-tl:1.12.2");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.slf4j:slf4j-api:1.7.32");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.commons:commons-lang3:3.3.2");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.poi:poi-ooxml:5.2.2");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.poi:poi:5.2.2");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("commons-codec:commons-codec:1.15");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.commons:commons-math3:3.6.1");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("com.zaxxer:SparseBitSet:1.2");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.poi:poi-ooxml-lite:5.2.2");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlbeans:xmlbeans:5.0.3");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.commons:commons-compress:1.21");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("commons-io:commons-io:2.11.0");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("com.github.virtuald:curvesapi:1.07");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.logging.log4j:log4j-api:2.17.2");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.commons:commons-collections4:4.4");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-transcoder:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-anim:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-css:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-ext:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-parser:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-svg-dom:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-awt-util:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:xmlgraphics-commons:2.9");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("commons-logging:commons-logging:1.0.4");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-bridge:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-script:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-dom:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("xml-apis:xml-apis:1.4.01");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-gvt:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-shared-resources:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-svggen:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-util:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-constants:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-i18n:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-xml:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("xml-apis:xml-apis-ext:1.3.04");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.apache.xmlgraphics:batik-codec:1.17");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("com.deepoove:poi-tl-plugin-markdown:1.0.3");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.commonmark:commonmark:0.17.1");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.commonmark:commonmark-ext-gfm-tables:0.17.1");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("com.deepoove:poi-tl-plugin-highlight:1.0.0");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("com.deepoove:codehighlight:1.0.3");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("com.deepoove:poi-tl-jsonmodel-support:1.0.0");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("com.google.code.gson:gson:2.9.0");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("org.danilopianini:gson-extras:0.4.0");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("javax.annotation:jsr250-api:1.0");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
  let dbx_artifact = MavenArtifact::from("net.jodah:typetools:0.6.3");
  jvm.deploy_artifact(&dbx_artifact).unwrap();
}