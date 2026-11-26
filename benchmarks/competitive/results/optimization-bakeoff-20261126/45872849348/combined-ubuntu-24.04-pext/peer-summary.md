| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.47x | 1.88x | 1.63x | 2.48x | 1.91x | 30/30 |
| materialize | 1.09x | 2.27x | 1.75x | 1.75x | 2.79x | 1.90x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.65x | 1.46x | 1.15x | 2.24x | 1.77x | 29/30 |
| perft_full_d3 | 1.01x | 1.36x | 1.55x | 1.52x | 1.94x | 1.62x | 30/30 |
| fen_parse | 1.01x | 1.27x | 1.58x | 1.16x | 1.65x | 1.14x | 30/30 |
| fen_write_native_ep | 1.01x | 4.55x | 10.07x | 1.93x | 28.15x | 2.24x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.34x | 4.00x | 1.33x | 1.34x | 30/30 |
| san_parse_all | 0.91x | 1.70x | N/A | 3.81x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 8.98x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 1.90x | 1.19x | 3.66x | 1.12x | 1.54x | 30/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.07x | N/A | N/A | 4/6 |
| startpos | 1.12x | 1.25x | 99.73x | 184.02x | 253.09x | 178.12x | 5/5 |
| chess960_materialize | 1.07x | 1.65x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.43x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.34x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 24.14x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 5.10x | 1.88x | 1.06x | 2.32x | 2.14x | 10/10 |
| corpus_materialize | 1.01x | 1.42x | 1.30x | 1.11x | 1.73x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.24x | 2.91x | 2.21x | 2.85x | 1.54x | 10/10 |
| replay_trusted_native | 1.13x | 1.00x | 1.86x | 1.84x | 2.55x | 2.93x | 9/10 |
| replay_checked_native | 1.02x | 2.20x | 1.19x | 9.48x | 8.98x | 10.60x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
