| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.81x | 1.50x | 1.53x | 1.97x | 1.92x | 30/30 |
| materialize | 0.97x | 1.79x | 1.48x | 1.03x | 2.27x | 1.34x | 28/30 |
| perft_bulk_d4 | 0.94x | 4.12x | 1.51x | 1.33x | 1.96x | 1.84x | 30/30 |
| perft_full_d3 | 1.00x | 1.67x | 1.70x | 1.40x | 1.97x | 2.01x | 30/30 |
| fen_parse | 0.99x | 1.24x | 1.28x | 1.34x | 1.61x | 1.56x | 30/30 |
| fen_write_native_ep | 0.88x | 2.80x | 6.07x | 1.18x | 19.96x | 1.60x | 29/30 |
| snapshot | 1.03x | 1.54x | 1.09x | 2.41x | 1.11x | 0.91x | 22/30 |
| san_parse_all | 1.01x | 1.97x | N/A | 4.07x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 11.80x | N/A | 2.47x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.03x | 1.17x | 5.38x | 1.06x | 1.27x | 25/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.13x | N/A | N/A | 5/6 |
| startpos | 1.12x | 1.38x | 67.95x | 133.61x | 154.33x | 143.05x | 5/5 |
| chess960_materialize | 1.25x | 1.53x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.11x | 3.25x | 1.65x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 37.85x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.10x | 21.34x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.25x | 6.63x | 2.27x | 1.28x | 2.58x | 3.11x | 10/10 |
| corpus_materialize | 0.99x | 1.45x | 1.17x | 1.12x | 1.66x | 1.15x | 7/10 |
| corpus_uci_sorted | 1.00x | 1.67x | 2.40x | 1.72x | 2.42x | 1.34x | 10/10 |
| replay_trusted_native | 0.94x | 0.61x | 1.48x | 1.59x | 1.58x | 1.53x | 8/10 |
| replay_checked_native | 1.04x | 2.24x | 0.90x | 7.89x | 6.91x | 8.25x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
