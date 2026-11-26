| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 3.91x | 1.68x | 1.42x | 2.14x | 1.75x | 30/30 |
| materialize | 0.96x | 2.02x | 1.63x | 1.66x | 2.51x | 1.83x | 30/30 |
| perft_bulk_d4 | 0.85x | 3.31x | 1.37x | 1.04x | 2.11x | 1.65x | 28/30 |
| perft_full_d3 | 0.96x | 1.45x | 1.67x | 1.62x | 2.39x | 1.79x | 30/30 |
| fen_parse | 0.98x | 1.38x | 1.36x | 1.12x | 1.57x | 1.24x | 30/30 |
| fen_write_native_ep | 1.00x | 4.62x | 10.49x | 1.94x | 30.89x | 2.58x | 30/30 |
| snapshot | 1.41x | 2.00x | 1.00x | 4.66x | 1.00x | 1.33x | 21/30 |
| san_parse_all | 0.88x | 1.72x | N/A | 3.83x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.24x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.94x | 1.20x | 3.64x | 1.24x | 1.51x | 30/30 |
| make_unmake_full | 0.83x | N/A | N/A | 0.94x | N/A | N/A | 1/6 |
| startpos | 1.22x | 1.25x | 98.86x | 188.85x | 235.25x | 199.22x | 5/5 |
| chess960_materialize | 0.99x | 1.56x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.37x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.03x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 18.95x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.66x | 4.27x | 1.64x | 0.89x | 1.99x | 1.81x | 8/10 |
| corpus_materialize | 0.94x | 1.33x | 1.26x | 1.04x | 1.63x | 1.14x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.24x | 3.07x | 2.26x | 2.93x | 1.58x | 10/10 |
| replay_trusted_native | 0.94x | 0.78x | 1.33x | 1.37x | 1.93x | 2.23x | 8/10 |
| replay_checked_native | 1.00x | 2.08x | 1.06x | 8.80x | 8.39x | 10.03x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
