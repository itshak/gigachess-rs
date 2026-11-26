| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.00x | 1.67x | 1.44x | 1.99x | 1.74x | 30/30 |
| materialize | 1.01x | 1.95x | 1.33x | 1.51x | 2.27x | 1.58x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.81x | 1.54x | 1.18x | 2.12x | 1.83x | 30/30 |
| perft_full_d3 | 1.05x | 0.89x | 0.95x | 0.89x | 1.22x | 1.04x | 12/30 |
| fen_parse | 1.00x | 1.30x | 1.65x | 1.44x | 1.89x | 1.56x | 30/30 |
| fen_write_native_ep | 1.01x | 3.13x | 6.12x | 1.33x | 23.53x | 1.67x | 30/30 |
| snapshot | 0.66x | 1.51x | 1.00x | 3.26x | 1.00x | 1.00x | 19/30 |
| san_parse_all | 1.04x | 1.64x | N/A | 5.03x | N/A | N/A | 12/12 |
| san_render_native | 1.14x | 9.33x | N/A | 3.98x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.59x | 1.06x | 7.94x | 1.15x | 1.43x | 28/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.00x | N/A | N/A | 4/6 |
| startpos | 2.21x | 1.17x | 65.50x | 137.90x | 177.33x | 146.19x | 5/5 |
| chess960_materialize | 1.03x | 1.47x | 1.27x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.29x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 42.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.05x | 23.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 6.32x | 2.44x | 1.27x | 2.78x | 2.71x | 10/10 |
| corpus_materialize | 1.05x | 1.42x | 1.24x | 1.10x | 1.74x | 1.22x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.09x | 1.32x | 1.17x | 1.31x | 0.94x | 8/10 |
| replay_trusted_native | 1.08x | 0.61x | 1.17x | 1.21x | 1.60x | 1.83x | 7/10 |
| replay_checked_native | 1.08x | 2.01x | 0.99x | 7.99x | 7.75x | 9.15x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
