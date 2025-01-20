window.BENCHMARK_DATA = {
  "lastUpdate": 1737359571073,
  "repoUrl": "https://github.com/contentstech-com/lazycsv",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "name": "contentstech-com",
            "username": "contentstech-com"
          },
          "committer": {
            "name": "contentstech-com",
            "username": "contentstech-com"
          },
          "id": "990c335fa9f71bcc5483086a5f81ec5422431417",
          "message": "add benchmark action",
          "timestamp": "2025-01-17T08:36:51Z",
          "url": "https://github.com/contentstech-com/lazycsv/pull/6/commits/990c335fa9f71bcc5483086a5f81ec5422431417"
        },
        "date": 1737108552774,
        "tool": "cargo",
        "benches": [
          {
            "name": "Parsers/lazy_csv",
            "value": 247252926,
            "range": "± 3047810",
            "unit": "ns/iter"
          },
          {
            "name": "Parsers/lazy_csv (into_rows)",
            "value": 257721484,
            "range": "± 1945549",
            "unit": "ns/iter"
          },
          {
            "name": "Parsers/lazy_csv (raw)",
            "value": 162092021,
            "range": "± 428499",
            "unit": "ns/iter"
          },
          {
            "name": "Parsers/lazy_csv (into_rows, raw)",
            "value": 160892187,
            "range": "± 223656",
            "unit": "ns/iter"
          },
          {
            "name": "Parsers/csv",
            "value": 286617271,
            "range": "± 1072229",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@xiniha.dev",
            "name": "Iha Shin",
            "username": "XiNiHa"
          },
          "committer": {
            "email": "me@xiniha.dev",
            "name": "Iha Shin",
            "username": "XiNiHa"
          },
          "distinct": true,
          "id": "fdda7e4bee12109ff6deea9d02fceb5f9db7ffa2",
          "message": "auto push benchmark results",
          "timestamp": "2025-01-20T16:50:42+09:00",
          "tree_id": "1cb3c88a99fb94872ff5cf15eb7cd2cb2f98f162",
          "url": "https://github.com/contentstech-com/lazycsv/commit/fdda7e4bee12109ff6deea9d02fceb5f9db7ffa2"
        },
        "date": 1737359570769,
        "tool": "cargo",
        "benches": [
          {
            "name": "Parsers/lazy_csv",
            "value": 246050038,
            "range": "± 2341131",
            "unit": "ns/iter"
          },
          {
            "name": "Parsers/lazy_csv (into_rows)",
            "value": 258548399,
            "range": "± 1760358",
            "unit": "ns/iter"
          },
          {
            "name": "Parsers/lazy_csv (raw)",
            "value": 161711930,
            "range": "± 499982",
            "unit": "ns/iter"
          },
          {
            "name": "Parsers/lazy_csv (into_rows, raw)",
            "value": 161235114,
            "range": "± 2581823",
            "unit": "ns/iter"
          },
          {
            "name": "Parsers/csv",
            "value": 286064198,
            "range": "± 827690",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}