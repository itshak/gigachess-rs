| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.89x | 1.49x | 1.60x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.82x | 1.44x | 1.26x | 2.02x | 1.39x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.41x | 1.40x | 1.27x | 1.79x | 1.84x | 30/30 |
| perft_full_d3 | 1.02x | 1.18x | 1.28x | 1.07x | 1.58x | 1.49x | 29/30 |
| fen_parse | 0.99x | 1.24x | 1.33x | 1.27x | 1.76x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.54x | 9.37x | 1.61x | 30.11x | 2.42x | 30/30 |
| snapshot | 0.98x | 1.15x | 1.01x | 1.46x | 1.00x | 0.94x | 19/30 |
| san_parse_all | 1.00x | 1.86x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.79x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 1.14x | 1.37x | 1.61x | 4.41x | 1.33x | 1.32x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.50x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.28x | 76.63x | 129.19x | 169.02x | 135.82x | 5/5 |
| chess960_materialize | 1.05x | 1.58x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.72x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.24x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 18.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.84x | 3.96x | 1.51x | 1.21x | 1.77x | 1.97x | 10/10 |
| corpus_materialize | 0.98x | 1.54x | 1.46x | 1.17x | 1.83x | 1.32x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.92x | 2.08x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 1.15x | 0.61x | 1.57x | 1.92x | 2.15x | 2.17x | 8/10 |
| replay_checked_native | 1.07x | 2.18x | 1.04x | 7.10x | 6.28x | 8.17x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
