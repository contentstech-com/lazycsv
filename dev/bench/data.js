window.BENCHMARK_DATA = {
  "lastUpdate": 1737108553063,
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
      }
    ]
  }
}
