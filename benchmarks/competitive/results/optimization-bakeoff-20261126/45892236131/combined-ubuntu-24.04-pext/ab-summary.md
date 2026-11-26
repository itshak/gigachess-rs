| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.952x | 0.920x | 1.408x |
| materialize | 0.974x | 0.949x | 1.242x |
| perft_bulk_d4 | 0.784x | 0.711x | 1.069x |
| perft_full_d3 | 0.838x | 0.792x | 1.097x |
| fen_parse | 0.961x | 0.896x | 1.232x |
| fen_write_native_ep | 0.941x | 0.881x | 1.621x |
| snapshot | 1.000x | 0.999x | 1.250x |
| san_parse_all | 0.882x | 0.857x | 1.846x |
| san_render_native | 0.982x | 0.894x | 2.157x |
| apply_children_full | 0.842x | 0.743x | 1.169x |
| make_unmake_full | 0.779x | 0.700x | 1.000x |
| startpos | 1.502x | 1.502x | 1.876x |
| chess960_materialize | 0.949x | 0.930x | 1.382x |
| chess960_perft_d3 | 0.843x | 0.771x | 1.256x |
| polyglot_key | 1.146x | 1.083x | 31.259x |
| polyglot_recompute | 0.974x | 0.967x | 1.009x |
| repetition_key | 1.086x | 1.083x | 22.572x |
| corpus_count | 0.957x | 0.949x | 1.360x |
| corpus_materialize | 0.981x | 0.975x | 1.174x |
| corpus_uci_sorted | 1.025x | 1.023x | 1.794x |
| replay_trusted_native | 0.752x | 0.655x | 0.688x |
| replay_checked_native | 0.814x | 0.748x | 0.908x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.00x).
