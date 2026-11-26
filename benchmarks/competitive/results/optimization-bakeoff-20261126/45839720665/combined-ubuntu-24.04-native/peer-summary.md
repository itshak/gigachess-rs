| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.23x | 1.82x | 1.53x | 2.35x | 1.86x | 30/30 |
| materialize | 1.10x | 2.23x | 1.75x | 1.72x | 2.68x | 1.90x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.77x | 1.55x | 1.17x | 2.32x | 1.85x | 29/30 |
| perft_full_d3 | 1.00x | 1.27x | 1.47x | 1.42x | 1.89x | 1.59x | 30/30 |
| fen_parse | 0.94x | 1.39x | 1.47x | 1.06x | 1.36x | 1.16x | 29/30 |
| fen_write_native_ep | 1.00x | 4.51x | 10.08x | 1.92x | 28.19x | 2.54x | 30/30 |
| snapshot | 1.01x | 1.83x | 1.15x | 4.33x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.03x | 1.96x | N/A | 4.41x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.03x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.14x | 1.33x | 4.03x | 1.34x | 1.68x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.12x | 88.53x | 162.65x | 197.09x | 177.97x | 5/5 |
| chess960_materialize | 0.99x | 1.66x | 1.63x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.54x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 38.27x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 23.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.21x | 1.97x | 1.07x | 2.37x | 2.18x | 9/10 |
| corpus_materialize | 1.00x | 1.41x | 1.30x | 1.07x | 1.73x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.28x | 2.95x | 2.28x | 2.79x | 1.57x | 10/10 |
| replay_trusted_native | 1.07x | 0.92x | 1.68x | 1.68x | 2.33x | 2.75x | 9/10 |
| replay_checked_native | 1.12x | 2.21x | 1.24x | 9.40x | 9.04x | 10.76x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
