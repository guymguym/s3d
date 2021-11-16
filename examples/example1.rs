use std::process::Command;

const S3D_ENDPOINT: &'static str = "http://localhost:3000";

fn main() {
    s3("ls", vec![]);
    s3api("head-bucket", vec!["--bucket", "lala"]);

    // s3("mb", vec!["s3://lala"]);
    // s3("ls", vec![]);
    // s3("ls", vec!["s3://lala"]);

    curl("GET", "/", vec![]);
    curl("HEAD", "/", vec![]);
    // curl("PUT", "/lala", vec![]);
    // curl("GET", "/lala", vec![]);
    // curl("HEAD", "/lala", vec![]);

    // # test
    // s3("ls", vec!["s3://lala"]);
    // s3("cp", vec!["README.md", "s3://lala/README.md"]);
    // s3("cp", vec!["s3://lala/README.md", "-"]);

    // # cleanup
    // s3("rb", vec!["s3://lala"]);
    // s3("ls", vec![]);
    // curl("DELETE", "/lala", vec![]);
    // curl("GET", "/", vec![]);
}

fn curl(method: &str, path: &str, args: Vec<&str>) {
    println!();
    println!("*** curl {} {} {:?}", method, path, args);
    println!();
    let mut cmd = Command::new("curl");
    cmd.arg("-s").arg("-i");
    if method == "HEAD" {
        cmd.arg("-I");
    } else {
        cmd.arg("-X").arg(method);
    }
    cmd.arg(format!("{}{}", S3D_ENDPOINT, path))
        .args(args)
        .status()
        .expect("Command 'curl' failed");
    println!();
}

fn s3(cmd: &str, args: Vec<&str>) {
    println!();
    println!("*** aws s3 {} {:?}", cmd, args);
    println!();
    Command::new("aws")
        .arg("--endpoint")
        .arg(S3D_ENDPOINT)
        .arg("s3")
        .arg(cmd)
        .args(args)
        .status()
        .expect("Command 'aws s3' failed");
    println!();
}

fn s3api(cmd: &str, args: Vec<&str>) {
    println!();
    println!("*** aws s3api {} {:?}", cmd, args);
    println!();
    Command::new("aws")
        .arg("--endpoint")
        .arg(S3D_ENDPOINT)
        .arg("s3api")
        .arg(cmd)
        .args(args)
        .status()
        .expect("Command 'aws s3api' failed");
    println!();
}
