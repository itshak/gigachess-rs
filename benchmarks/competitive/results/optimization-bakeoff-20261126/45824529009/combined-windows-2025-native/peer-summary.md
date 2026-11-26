| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.35x | 1.51x | 1.47x | 2.11x | 1.88x | 30/30 |
| materialize | 1.01x | 1.92x | 1.29x | 1.20x | 2.22x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.66x | 1.45x | 1.22x | 2.06x | 1.86x | 30/30 |
| perft_full_d3 | 0.97x | 1.30x | 1.37x | 1.23x | 1.81x | 1.76x | 30/30 |
| fen_parse | 1.02x | 1.34x | 1.44x | 1.31x | 2.20x | 1.49x | 30/30 |
| fen_write_native_ep | 1.01x | 4.19x | 8.12x | 1.56x | 32.38x | 1.96x | 30/30 |
| snapshot | 1.01x | 1.88x | 1.01x | 4.10x | 1.01x | 1.01x | 27/30 |
| san_parse_all | 1.00x | 2.06x | N/A | 5.56x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.11x | N/A | 3.85x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.17x | 1.33x | 9.20x | 1.40x | 1.89x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.93x | 153.45x | 229.44x | 173.14x | 5/5 |
| chess960_materialize | 1.01x | 1.55x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.68x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 24.87x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.20x | 18.59x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 4.95x | 1.62x | 1.23x | 2.24x | 2.22x | 10/10 |
| corpus_materialize | 1.07x | 1.50x | 1.17x | 1.10x | 1.84x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.75x | 2.21x | 1.81x | 2.15x | 1.41x | 10/10 |
| replay_trusted_native | 1.00x | 0.83x | 1.57x | 2.06x | 2.56x | 2.93x | 9/10 |
| replay_checked_native | 0.98x | 2.25x | 0.98x | 7.73x | 7.50x | 9.19x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
