| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.64x | 1.39x | 1.53x | 1.81x | 1.80x | 30/30 |
| materialize | 1.00x | 1.78x | 1.40x | 1.22x | 1.96x | 1.35x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.56x | 1.46x | 1.33x | 1.86x | 1.91x | 30/30 |
| perft_full_d3 | 0.97x | 0.81x | 0.88x | 0.73x | 1.07x | 1.02x | 9/30 |
| fen_parse | 1.04x | 1.28x | 1.38x | 1.32x | 1.81x | 1.44x | 30/30 |
| fen_write_native_ep | 1.00x | 3.04x | 6.19x | 1.04x | 19.83x | 1.59x | 28/30 |
| snapshot | 1.01x | 1.20x | 1.01x | 1.65x | 1.03x | 1.01x | 27/30 |
| san_parse_all | 0.98x | 1.75x | N/A | 4.11x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.10x | N/A | 2.39x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 0.88x | 1.04x | 2.85x | 0.85x | 0.85x | 9/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.10x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.04x | 63.50x | 108.24x | 143.03x | 114.14x | 5/5 |
| chess960_materialize | 1.00x | 1.40x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.38x | 1.21x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 26.18x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.09x | 20.00x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.66x | 1.82x | 1.43x | 2.04x | 2.29x | 10/10 |
| corpus_materialize | 0.98x | 1.57x | 1.51x | 1.18x | 1.83x | 1.34x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.40x | 1.94x | 1.38x | 1.87x | 0.97x | 8/10 |
| replay_trusted_native | 0.98x | 0.41x | 1.07x | 1.28x | 1.42x | 1.48x | 8/10 |
| replay_checked_native | 0.98x | 1.80x | 0.87x | 5.74x | 5.20x | 6.79x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
