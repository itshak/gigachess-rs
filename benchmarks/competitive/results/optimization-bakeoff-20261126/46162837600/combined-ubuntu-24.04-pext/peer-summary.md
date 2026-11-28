| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.70x | 1.97x | 1.71x | 2.62x | 2.01x | 30/30 |
| materialize | 1.01x | 2.30x | 1.84x | 1.77x | 2.94x | 1.93x | 30/30 |
| perft_bulk_d4 | 1.03x | 3.94x | 1.57x | 1.23x | 2.43x | 1.91x | 29/30 |
| perft_full_d3 | 1.23x | 1.65x | 1.88x | 1.86x | 2.43x | 2.01x | 30/30 |
| fen_parse | 0.95x | 1.21x | 1.32x | 1.13x | 1.50x | 1.19x | 30/30 |
| fen_write_native_ep | 1.02x | 4.62x | 10.53x | 2.08x | 31.12x | 2.23x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.12x | 1.90x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.04x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 2.08x | 1.32x | 3.97x | 1.31x | 1.64x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 88.74x | 164.46x | 206.41x | 174.16x | 5/5 |
| chess960_materialize | 0.96x | 1.67x | 1.64x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.54x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 39.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.69x | 1.63x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 18.67x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.17x | 5.98x | 2.22x | 1.24x | 2.73x | 2.49x | 10/10 |
| corpus_materialize | 1.00x | 1.43x | 1.29x | 1.09x | 1.73x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.27x | 2.99x | 2.23x | 2.96x | 1.55x | 10/10 |
| replay_trusted_native | 1.12x | 0.95x | 1.66x | 1.76x | 2.41x | 2.78x | 9/10 |
| replay_checked_native | 0.98x | 2.12x | 1.11x | 9.06x | 8.67x | 10.25x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
