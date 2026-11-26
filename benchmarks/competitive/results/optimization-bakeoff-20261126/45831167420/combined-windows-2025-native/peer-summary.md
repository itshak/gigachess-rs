| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.13x | 4.98x | 1.70x | 1.68x | 2.41x | 2.13x | 30/30 |
| materialize | 1.14x | 2.03x | 1.38x | 1.27x | 2.37x | 1.58x | 30/30 |
| perft_bulk_d4 | 1.18x | 4.07x | 1.63x | 1.39x | 2.35x | 2.15x | 30/30 |
| perft_full_d3 | 1.00x | 1.36x | 1.45x | 1.28x | 1.92x | 1.83x | 30/30 |
| fen_parse | 1.15x | 1.31x | 1.38x | 1.35x | 2.22x | 1.44x | 30/30 |
| fen_write_native_ep | 1.03x | 4.30x | 8.14x | 1.56x | 31.82x | 2.00x | 30/30 |
| snapshot | 1.04x | 1.81x | 0.99x | 4.22x | 0.99x | 1.13x | 22/30 |
| san_parse_all | 1.02x | 2.07x | N/A | 5.47x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.93x | N/A | 3.75x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.28x | 1.41x | 9.59x | 1.47x | 1.63x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 82.13x | 169.05x | 249.82x | 176.91x | 5/5 |
| chess960_materialize | 0.98x | 1.64x | 1.29x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.89x | 1.56x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 28.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 21.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 5.37x | 1.88x | 1.39x | 2.53x | 2.43x | 10/10 |
| corpus_materialize | 1.01x | 1.61x | 1.34x | 1.15x | 1.98x | 1.34x | 10/10 |
| corpus_uci_sorted | 0.93x | 1.79x | 2.14x | 1.94x | 2.17x | 1.41x | 10/10 |
| replay_trusted_native | 0.93x | 0.80x | 1.60x | 2.11x | 2.45x | 3.13x | 8/10 |
| replay_checked_native | 0.91x | 2.18x | 0.97x | 7.18x | 7.41x | 9.34x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
