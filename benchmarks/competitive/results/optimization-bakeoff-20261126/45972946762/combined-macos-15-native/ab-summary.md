| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.033x | 0.838x | 1.427x |
| materialize | 0.959x | 0.671x | 1.193x |
| perft_bulk_d4 | 1.108x | 0.835x | 1.281x |
| perft_full_d3 | 1.023x | 0.881x | 1.350x |
| fen_parse | 0.992x | 0.711x | 1.149x |
| fen_write_native_ep | 0.937x | 0.748x | 1.720x |
| snapshot | 1.217x | 0.751x | 0.918x |
| san_parse_all | 0.997x | 0.906x | 1.810x |
| san_render_native | 0.946x | 0.874x | 2.711x |
| apply_children_full | 0.968x | 0.852x | 1.106x |
| make_unmake_full | 0.893x | 0.762x | 1.224x |
| startpos | 1.215x | 1.215x | 1.986x |
| chess960_materialize | 1.097x | 0.798x | 1.511x |
| chess960_perft_d3 | 0.983x | 0.917x | 1.535x |
| polyglot_key | 0.912x | 0.856x | 36.036x |
| polyglot_recompute | 0.790x | 0.633x | 1.161x |
| repetition_key | 0.830x | 0.803x | 19.716x |
| corpus_count | 0.976x | 0.896x | 1.418x |
| corpus_materialize | 0.870x | 0.838x | 1.125x |
| corpus_uci_sorted | 0.934x | 0.782x | 1.336x |
| replay_trusted_native | 0.886x | 0.880x | 0.752x |
| replay_checked_native | 0.892x | 0.842x | 1.010x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.00x).
