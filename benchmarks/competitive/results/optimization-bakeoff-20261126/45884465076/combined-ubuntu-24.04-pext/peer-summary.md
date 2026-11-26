| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.48x | 1.83x | 1.62x | 2.42x | 1.90x | 30/30 |
| materialize | 1.02x | 2.07x | 1.60x | 1.67x | 2.51x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.80x | 1.51x | 1.18x | 2.20x | 1.83x | 29/30 |
| perft_full_d3 | 1.00x | 1.37x | 1.64x | 1.50x | 1.90x | 1.64x | 30/30 |
| fen_parse | 1.00x | 1.32x | 1.47x | 1.18x | 1.65x | 1.39x | 30/30 |
| fen_write_native_ep | 0.95x | 4.67x | 9.73x | 1.84x | 29.18x | 2.44x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.34x | 4.00x | 1.33x | 1.35x | 30/30 |
| san_parse_all | 1.00x | 1.78x | N/A | 4.08x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.47x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.09x | 1.34x | 4.11x | 1.25x | 1.74x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 102.44x | 174.13x | 236.49x | 199.37x | 5/5 |
| chess960_materialize | 1.00x | 1.64x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.58x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.04x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.10x | 32.44x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.81x | 1.88x | 0.96x | 2.16x | 2.02x | 8/10 |
| corpus_materialize | 0.99x | 1.39x | 1.26x | 1.06x | 1.68x | 1.14x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.27x | 2.98x | 2.18x | 2.90x | 1.55x | 10/10 |
| replay_trusted_native | 0.94x | 0.83x | 1.50x | 1.47x | 2.06x | 2.33x | 9/10 |
| replay_checked_native | 0.99x | 2.33x | 1.15x | 9.31x | 8.94x | 10.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
