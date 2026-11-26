| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.15x | 4.60x | 1.85x | 1.67x | 2.27x | 2.03x | 30/30 |
| materialize | 1.04x | 1.95x | 1.34x | 1.43x | 2.34x | 1.64x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.97x | 1.54x | 1.40x | 2.14x | 1.94x | 30/30 |
| perft_full_d3 | 1.02x | 1.64x | 1.65x | 1.51x | 2.14x | 1.91x | 30/30 |
| fen_parse | 0.97x | 1.36x | 1.53x | 1.66x | 2.09x | 1.71x | 30/30 |
| fen_write_native_ep | 0.95x | 4.24x | 7.91x | 1.61x | 33.13x | 2.07x | 30/30 |
| snapshot | 1.00x | 1.96x | 1.18x | 3.84x | 1.09x | 1.12x | 29/30 |
| san_parse_all | 0.99x | 1.87x | N/A | 5.62x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 11.36x | N/A | 4.46x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.83x | 1.22x | 8.93x | 1.21x | 1.34x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 2.30x | 0.99x | 74.56x | 185.84x | 222.94x | 177.93x | 4/5 |
| chess960_materialize | 1.06x | 1.57x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.15x | 2.48x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 36.71x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 15.64x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.14x | 6.50x | 2.57x | 1.51x | 2.83x | 2.88x | 10/10 |
| corpus_materialize | 1.01x | 1.45x | 1.26x | 1.12x | 1.79x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.72x | 2.09x | 1.81x | 2.07x | 1.42x | 10/10 |
| replay_trusted_native | 1.03x | 0.95x | 1.48x | 1.70x | 2.22x | 2.43x | 9/10 |
| replay_checked_native | 1.03x | 2.08x | 1.03x | 7.33x | 6.93x | 8.63x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
