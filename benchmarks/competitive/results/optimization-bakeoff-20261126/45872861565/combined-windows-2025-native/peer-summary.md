| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 5.19x | 1.82x | 1.66x | 2.48x | 2.13x | 30/30 |
| materialize | 0.94x | 2.01x | 1.38x | 1.32x | 2.37x | 1.56x | 30/30 |
| perft_bulk_d4 | 0.92x | 4.05x | 1.59x | 1.33x | 2.35x | 2.03x | 29/30 |
| perft_full_d3 | 1.13x | 1.66x | 1.80x | 1.51x | 2.25x | 2.19x | 30/30 |
| fen_parse | 0.85x | 1.19x | 1.23x | 1.13x | 2.10x | 1.37x | 30/30 |
| fen_write_native_ep | 0.94x | 4.32x | 7.99x | 1.54x | 31.83x | 2.02x | 30/30 |
| snapshot | 0.99x | 1.98x | 1.06x | 4.52x | 1.09x | 1.15x | 28/30 |
| san_parse_all | 0.92x | 2.06x | N/A | 5.29x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 11.25x | N/A | 3.90x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.33x | 1.43x | 9.93x | 1.55x | 1.67x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 80.68x | 161.38x | 262.34x | 183.59x | 5/5 |
| chess960_materialize | 0.82x | 1.35x | 1.10x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.97x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 28.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 20.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 5.78x | 1.79x | 1.47x | 2.48x | 2.37x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.28x | 1.15x | 1.98x | 1.42x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.72x | 2.15x | 1.81x | 2.12x | 1.41x | 10/10 |
| replay_trusted_native | 0.92x | 0.80x | 1.52x | 1.90x | 2.42x | 2.91x | 8/10 |
| replay_checked_native | 0.92x | 2.38x | 1.01x | 7.60x | 7.70x | 9.59x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
