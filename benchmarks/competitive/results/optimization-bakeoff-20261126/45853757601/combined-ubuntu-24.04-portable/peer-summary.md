| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.94x | 1.65x | 1.43x | 1.92x | 1.37x | 30/30 |
| materialize | 1.00x | 2.36x | 1.87x | 1.85x | 2.85x | 2.04x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.00x | 1.59x | 1.19x | 1.97x | 1.77x | 29/30 |
| perft_full_d3 | 1.00x | 1.37x | 1.60x | 1.42x | 1.91x | 1.59x | 30/30 |
| fen_parse | 1.00x | 1.57x | 1.40x | 1.14x | 1.46x | 1.17x | 30/30 |
| fen_write_native_ep | 0.99x | 3.11x | 10.10x | 1.84x | 28.00x | 2.28x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 1.01x | 1.90x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.25x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.15x | 1.42x | 4.09x | 1.32x | 1.54x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.36x | 61.91x | 103.71x | 119.82x | 97.38x | 5/5 |
| chess960_materialize | 1.06x | 1.85x | 1.81x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.13x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 40.46x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.90x | 19.18x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.31x | 2.18x | 1.18x | 2.19x | 2.95x | 10/10 |
| corpus_materialize | 1.00x | 1.51x | 1.41x | 1.15x | 1.85x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.97x | 3.08x | 2.26x | 2.95x | 1.55x | 10/10 |
| replay_trusted_native | 0.96x | 0.84x | 1.62x | 1.56x | 2.14x | 1.98x | 9/10 |
| replay_checked_native | 0.98x | 1.88x | 1.08x | 8.20x | 7.93x | 9.12x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
