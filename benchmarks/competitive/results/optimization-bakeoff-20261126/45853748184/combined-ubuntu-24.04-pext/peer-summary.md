| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.47x | 1.84x | 1.62x | 2.28x | 1.93x | 30/30 |
| materialize | 1.00x | 2.09x | 1.61x | 1.71x | 2.39x | 1.65x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.85x | 1.52x | 1.19x | 2.18x | 1.87x | 29/30 |
| perft_full_d3 | 1.00x | 1.38x | 1.55x | 1.48x | 1.94x | 1.65x | 30/30 |
| fen_parse | 0.90x | 1.14x | 1.37x | 0.97x | 1.36x | 1.14x | 25/30 |
| fen_write_native_ep | 0.95x | 4.60x | 9.17x | 2.03x | 26.43x | 2.63x | 30/30 |
| snapshot | 1.00x | 1.85x | 1.17x | 4.31x | 1.17x | 1.33x | 30/30 |
| san_parse_all | 1.05x | 1.81x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.36x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 2.01x | 1.28x | 3.95x | 1.27x | 1.61x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.11x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.11x | 91.85x | 147.22x | 197.14x | 160.15x | 5/5 |
| chess960_materialize | 1.00x | 1.65x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.53x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.81x | 25.18x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.80x | 1.84x | 0.95x | 2.15x | 2.03x | 8/10 |
| corpus_materialize | 1.00x | 1.43x | 1.30x | 1.09x | 1.71x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.26x | 2.93x | 2.25x | 2.84x | 1.56x | 10/10 |
| replay_trusted_native | 0.96x | 0.85x | 1.48x | 1.47x | 2.08x | 2.35x | 9/10 |
| replay_checked_native | 0.98x | 2.13x | 1.04x | 8.59x | 8.19x | 9.50x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
