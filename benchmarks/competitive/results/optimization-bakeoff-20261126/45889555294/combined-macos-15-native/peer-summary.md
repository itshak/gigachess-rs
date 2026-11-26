| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.57x | 1.58x | 1.68x | 2.29x | 1.92x | 30/30 |
| materialize | 0.95x | 2.08x | 1.56x | 1.10x | 2.17x | 1.34x | 27/30 |
| perft_bulk_d4 | 0.92x | 3.71x | 1.45x | 1.21x | 1.81x | 1.80x | 29/30 |
| perft_full_d3 | 0.75x | 1.18x | 1.29x | 0.98x | 1.49x | 1.49x | 27/30 |
| fen_parse | 1.06x | 1.22x | 1.25x | 1.24x | 1.54x | 1.41x | 30/30 |
| fen_write_native_ep | 0.96x | 3.79x | 7.86x | 1.61x | 24.92x | 2.04x | 30/30 |
| snapshot | 1.17x | 1.84x | 1.34x | 2.71x | 1.36x | 1.20x | 30/30 |
| san_parse_all | 1.12x | 1.78x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 1.09x | 13.59x | N/A | 3.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.10x | 1.33x | 1.50x | 6.68x | 1.41x | 1.56x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.11x | N/A | N/A | 4/6 |
| startpos | 2.49x | 2.58x | 183.35x | 292.72x | 337.32x | 297.69x | 5/5 |
| chess960_materialize | 1.09x | 1.72x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.94x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.13x | 41.18x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.01x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.22x | 21.75x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 5.12x | 1.70x | 1.06x | 2.15x | 2.19x | 9/10 |
| corpus_materialize | 1.08x | 1.49x | 1.35x | 1.14x | 1.88x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.12x | 1.82x | 2.55x | 1.82x | 2.40x | 1.35x | 10/10 |
| replay_trusted_native | 1.08x | 0.62x | 1.63x | 1.61x | 1.75x | 1.71x | 8/10 |
| replay_checked_native | 1.37x | 2.57x | 1.21x | 9.14x | 8.44x | 10.21x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
