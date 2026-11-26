| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.87x | 4.48x | 1.56x | 1.51x | 2.37x | 1.99x | 30/30 |
| materialize | 0.92x | 2.00x | 1.27x | 1.30x | 2.28x | 1.56x | 30/30 |
| perft_bulk_d4 | 0.91x | 3.88x | 1.56x | 1.29x | 2.37x | 2.06x | 29/30 |
| perft_full_d3 | 0.90x | 1.13x | 1.19x | 1.08x | 1.57x | 1.47x | 27/30 |
| fen_parse | 1.10x | 1.35x | 1.51x | 1.37x | 2.30x | 1.49x | 30/30 |
| fen_write_native_ep | 0.90x | 3.92x | 7.95x | 1.53x | 30.66x | 1.95x | 30/30 |
| snapshot | 0.93x | 2.06x | 1.04x | 4.09x | 1.05x | 1.03x | 21/30 |
| san_parse_all | 0.93x | 2.10x | N/A | 5.40x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 10.56x | N/A | 3.75x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 2.17x | 1.34x | 9.47x | 1.41x | 1.55x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.26x | 79.99x | 167.20x | 235.73x | 220.21x | 5/5 |
| chess960_materialize | 0.98x | 1.52x | 1.27x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 3.12x | 1.55x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 29.55x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 22.81x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.51x | 1.82x | 1.42x | 2.54x | 2.55x | 10/10 |
| corpus_materialize | 0.97x | 1.55x | 1.21x | 1.12x | 1.93x | 1.36x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.85x | 2.32x | 1.89x | 2.32x | 1.44x | 10/10 |
| replay_trusted_native | 1.00x | 0.85x | 1.58x | 2.11x | 2.60x | 3.07x | 9/10 |
| replay_checked_native | 1.04x | 2.39x | 1.09x | 8.47x | 8.24x | 9.63x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
