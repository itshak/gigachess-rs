| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.11x | 1.72x | 1.49x | 2.35x | 1.75x | 30/30 |
| materialize | 1.00x | 2.15x | 1.70x | 1.71x | 2.67x | 1.85x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.32x | 1.31x | 1.03x | 2.07x | 1.61x | 28/30 |
| perft_full_d3 | 0.99x | 1.47x | 1.62x | 1.62x | 2.15x | 1.99x | 30/30 |
| fen_parse | 1.07x | 1.47x | 1.38x | 1.18x | 1.57x | 1.29x | 30/30 |
| fen_write_native_ep | 0.96x | 4.33x | 9.97x | 1.99x | 29.21x | 2.44x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.68x | 1.00x | 1.33x | 24/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 4.48x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 9.00x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.90x | 1.20x | 3.83x | 1.27x | 1.66x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 0.81x | 1.00x | 78.46x | 140.27x | 183.70x | 164.82x | 5/5 |
| chess960_materialize | 0.98x | 1.57x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.36x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 40.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 17.27x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 4.64x | 1.70x | 0.97x | 2.14x | 1.94x | 8/10 |
| corpus_materialize | 1.01x | 1.38x | 1.27x | 1.09x | 1.69x | 1.15x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.22x | 2.98x | 2.22x | 2.92x | 1.52x | 10/10 |
| replay_trusted_native | 0.98x | 0.81x | 1.40x | 1.50x | 2.04x | 2.34x | 8/10 |
| replay_checked_native | 1.01x | 2.08x | 1.07x | 8.99x | 8.42x | 9.96x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
