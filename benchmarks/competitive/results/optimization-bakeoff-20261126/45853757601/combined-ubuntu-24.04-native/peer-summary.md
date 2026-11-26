| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.21x | 1.73x | 1.62x | 2.18x | 1.90x | 30/30 |
| materialize | 1.00x | 2.05x | 1.57x | 1.39x | 2.31x | 1.56x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.57x | 1.47x | 1.22x | 2.00x | 1.75x | 29/30 |
| perft_full_d3 | 1.00x | 1.42x | 1.55x | 1.38x | 1.88x | 1.81x | 30/30 |
| fen_parse | 0.92x | 1.22x | 1.40x | 1.05x | 1.57x | 1.23x | 30/30 |
| fen_write_native_ep | 0.96x | 4.92x | 9.53x | 1.83x | 27.65x | 2.54x | 30/30 |
| snapshot | 1.00x | 2.06x | 1.23x | 4.32x | 1.89x | 1.33x | 30/30 |
| san_parse_all | 1.02x | 1.95x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.16x | N/A | 2.40x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 2.07x | 1.23x | 4.52x | 1.25x | 1.47x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 99.16x | 181.90x | 249.49x | 197.05x | 5/5 |
| chess960_materialize | 1.00x | 1.72x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.51x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 31.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 19.50x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.22x | 1.64x | 0.96x | 1.93x | 1.88x | 9/10 |
| corpus_materialize | 1.02x | 1.51x | 1.43x | 1.12x | 1.81x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.23x | 2.91x | 2.20x | 2.78x | 1.52x | 10/10 |
| replay_trusted_native | 0.99x | 0.83x | 1.54x | 1.73x | 2.18x | 2.40x | 9/10 |
| replay_checked_native | 0.93x | 2.03x | 1.03x | 7.61x | 7.08x | 8.55x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
