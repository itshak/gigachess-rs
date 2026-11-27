| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 5.10x | 1.73x | 1.62x | 2.60x | 2.12x | 30/30 |
| materialize | 1.02x | 2.09x | 1.39x | 1.17x | 2.46x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.23x | 1.53x | 1.40x | 2.45x | 2.23x | 30/30 |
| perft_full_d3 | 1.03x | 1.44x | 1.55x | 1.30x | 2.06x | 2.09x | 30/30 |
| fen_parse | 1.02x | 1.39x | 1.57x | 1.56x | 2.26x | 1.64x | 30/30 |
| fen_write_native_ep | 1.02x | 3.70x | 7.11x | 1.50x | 33.16x | 2.03x | 30/30 |
| snapshot | 1.00x | 2.52x | 1.54x | 5.86x | 1.56x | 1.51x | 30/30 |
| san_parse_all | 1.04x | 1.96x | N/A | 4.64x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.21x | N/A | 4.01x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.89x | 1.39x | 9.03x | 1.43x | 1.93x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 74.17x | 136.43x | 195.09x | 148.17x | 4/5 |
| chess960_materialize | 1.03x | 1.74x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.90x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 35.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 28.01x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.82x | 1.72x | 1.17x | 2.27x | 2.21x | 9/10 |
| corpus_materialize | 1.02x | 1.63x | 1.29x | 1.12x | 2.23x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.62x | 1.99x | 1.75x | 1.94x | 1.37x | 10/10 |
| replay_trusted_native | 1.09x | 0.76x | 1.55x | 1.81x | 2.68x | 3.21x | 9/10 |
| replay_checked_native | 1.02x | 2.54x | 1.14x | 8.87x | 8.84x | 11.05x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
