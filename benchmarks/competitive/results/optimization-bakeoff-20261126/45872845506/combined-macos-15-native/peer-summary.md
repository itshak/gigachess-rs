| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.93x | 1.65x | 1.72x | 2.46x | 2.01x | 30/30 |
| materialize | 0.96x | 2.14x | 1.65x | 1.11x | 2.31x | 1.40x | 27/30 |
| perft_bulk_d4 | 0.97x | 4.03x | 1.49x | 1.22x | 1.93x | 1.84x | 30/30 |
| perft_full_d3 | 0.95x | 1.55x | 1.73x | 1.31x | 2.06x | 1.96x | 30/30 |
| fen_parse | 1.04x | 1.31x | 1.35x | 1.40x | 1.69x | 1.61x | 30/30 |
| fen_write_native_ep | 0.98x | 4.45x | 9.30x | 1.76x | 29.22x | 2.29x | 30/30 |
| snapshot | 0.98x | 1.67x | 1.11x | 2.38x | 1.12x | 1.03x | 25/30 |
| san_parse_all | 1.02x | 1.76x | N/A | 3.69x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 12.28x | N/A | 2.77x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.13x | 1.21x | 5.78x | 1.19x | 1.36x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.21x | 69.95x | 142.61x | 165.54x | 152.67x | 5/5 |
| chess960_materialize | 0.97x | 1.72x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 3.23x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 39.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 20.09x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.82x | 1.89x | 1.12x | 2.22x | 2.42x | 9/10 |
| corpus_materialize | 0.96x | 1.44x | 1.38x | 1.12x | 1.86x | 1.22x | 10/10 |
| corpus_uci_sorted | 0.94x | 1.82x | 2.54x | 1.87x | 2.43x | 1.36x | 10/10 |
| replay_trusted_native | 0.94x | 0.62x | 1.50x | 1.57x | 1.74x | 1.74x | 8/10 |
| replay_checked_native | 0.95x | 2.66x | 1.20x | 9.29x | 8.04x | 10.76x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
