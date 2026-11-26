| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 3.20x | 1.61x | 1.55x | 2.08x | 1.47x | 30/30 |
| materialize | 1.04x | 2.14x | 1.89x | 1.37x | 2.51x | 1.65x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.10x | 1.68x | 1.31x | 2.11x | 1.82x | 30/30 |
| perft_full_d3 | 1.00x | 1.35x | 1.57x | 1.30x | 1.89x | 1.75x | 30/30 |
| fen_parse | 0.99x | 1.32x | 1.37x | 1.16x | 1.94x | 1.14x | 30/30 |
| fen_write_native_ep | 1.04x | 2.91x | 9.40x | 1.66x | 27.42x | 2.14x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.23x | 2.90x | 1.03x | 0.99x | 25/30 |
| san_parse_all | 1.01x | 2.05x | N/A | 4.37x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 10.48x | N/A | 2.29x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.23x | 1.56x | 4.54x | 1.43x | 1.48x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 64.38x | 116.74x | 179.34x | 114.40x | 5/5 |
| chess960_materialize | 1.01x | 1.68x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.30x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 23.63x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 17.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 3.89x | 1.94x | 1.32x | 2.16x | 2.60x | 10/10 |
| corpus_materialize | 1.01x | 1.62x | 1.65x | 1.22x | 2.11x | 1.41x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.22x | 3.23x | 2.44x | 3.10x | 1.73x | 10/10 |
| replay_trusted_native | 0.93x | 0.83x | 1.61x | 1.87x | 2.50x | 2.33x | 9/10 |
| replay_checked_native | 0.96x | 2.21x | 1.09x | 7.78x | 7.88x | 8.87x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
