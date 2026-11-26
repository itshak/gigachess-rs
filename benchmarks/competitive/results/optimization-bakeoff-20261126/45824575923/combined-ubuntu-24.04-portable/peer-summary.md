| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 3.15x | 1.60x | 1.46x | 2.05x | 1.44x | 30/30 |
| materialize | 1.07x | 2.12x | 1.86x | 1.34x | 2.51x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.15x | 3.13x | 1.67x | 1.30x | 2.11x | 1.82x | 30/30 |
| perft_full_d3 | 1.03x | 1.49x | 1.72x | 1.38x | 2.05x | 1.83x | 30/30 |
| fen_parse | 1.04x | 1.62x | 1.47x | 1.23x | 2.28x | 1.28x | 30/30 |
| fen_write_native_ep | 1.01x | 2.95x | 9.85x | 1.70x | 27.84x | 2.03x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.09x | 3.06x | 1.01x | 1.07x | 28/30 |
| san_parse_all | 1.00x | 2.04x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.52x | N/A | 2.29x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.13x | 1.49x | 4.31x | 1.37x | 1.41x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.32x | 66.22x | 113.40x | 180.04x | 109.61x | 5/5 |
| chess960_materialize | 1.10x | 1.66x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.07x | 2.31x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 27.93x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.94x | 18.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 3.92x | 1.93x | 1.29x | 2.16x | 2.59x | 10/10 |
| corpus_materialize | 1.07x | 1.63x | 1.61x | 1.18x | 2.10x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.21x | 3.24x | 2.45x | 3.12x | 1.95x | 10/10 |
| replay_trusted_native | 1.01x | 0.88x | 1.73x | 1.95x | 2.57x | 2.61x | 9/10 |
| replay_checked_native | 1.00x | 2.18x | 1.05x | 7.27x | 7.43x | 8.44x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
