| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.05x | 1.71x | 1.44x | 2.25x | 1.75x | 30/30 |
| materialize | 0.96x | 2.16x | 1.67x | 1.67x | 2.62x | 1.86x | 30/30 |
| perft_bulk_d4 | 0.88x | 3.32x | 1.33x | 1.03x | 2.04x | 1.61x | 28/30 |
| perft_full_d3 | 0.93x | 1.26x | 1.49x | 1.41x | 1.92x | 1.58x | 30/30 |
| fen_parse | 1.00x | 1.36x | 1.38x | 1.06x | 1.49x | 1.27x | 30/30 |
| fen_write_native_ep | 0.91x | 4.08x | 8.88x | 1.72x | 26.15x | 2.41x | 30/30 |
| snapshot | 0.67x | 1.34x | 0.87x | 3.01x | 0.86x | 1.00x | 14/30 |
| san_parse_all | 0.97x | 1.84x | N/A | 4.15x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.80x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 2.05x | 1.31x | 3.81x | 1.31x | 1.56x | 30/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.08x | N/A | N/A | 5/6 |
| startpos | 0.99x | 1.31x | 87.95x | 152.97x | 191.60x | 173.55x | 5/5 |
| chess960_materialize | 0.92x | 1.70x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.87x | 2.15x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 37.75x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.11x | 21.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.24x | 1.96x | 1.09x | 2.39x | 2.15x | 9/10 |
| corpus_materialize | 1.02x | 1.42x | 1.30x | 1.09x | 1.71x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.32x | 2.95x | 2.26x | 2.88x | 1.62x | 10/10 |
| replay_trusted_native | 1.02x | 0.89x | 1.62x | 1.58x | 2.22x | 2.60x | 9/10 |
| replay_checked_native | 0.93x | 2.00x | 1.06x | 8.41x | 8.02x | 9.77x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
