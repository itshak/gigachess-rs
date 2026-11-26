| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.75x | 1.57x | 1.35x | 1.84x | 1.63x | 30/30 |
| materialize | 1.07x | 1.95x | 1.34x | 1.52x | 2.27x | 1.56x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.22x | 1.31x | 0.99x | 1.81x | 1.55x | 26/30 |
| perft_full_d3 | 1.01x | 1.56x | 1.73x | 1.58x | 2.22x | 1.81x | 30/30 |
| fen_parse | 1.11x | 1.35x | 1.53x | 1.36x | 1.85x | 1.55x | 30/30 |
| fen_write_native_ep | 1.02x | 4.14x | 8.26x | 1.62x | 30.91x | 2.17x | 30/30 |
| snapshot | 1.26x | 2.12x | 1.27x | 5.10x | 1.27x | 1.27x | 30/30 |
| san_parse_all | 0.98x | 1.64x | N/A | 4.92x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 9.67x | N/A | 4.22x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.02x | 1.30x | 10.22x | 1.24x | 1.62x | 30/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 0.96x | 1.01x | 77.72x | 158.30x | 209.98x | 176.15x | 5/5 |
| chess960_materialize | 1.06x | 1.48x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.18x | 2.52x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 43.33x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.08x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.93x | 24.56x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 4.01x | 1.64x | 0.82x | 1.79x | 1.71x | 8/10 |
| corpus_materialize | 1.04x | 1.35x | 1.17x | 1.03x | 1.70x | 1.22x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.78x | 2.18x | 1.82x | 2.09x | 1.50x | 10/10 |
| replay_trusted_native | 1.02x | 0.81x | 1.53x | 1.56x | 2.09x | 2.41x | 9/10 |
| replay_checked_native | 0.97x | 2.09x | 1.07x | 8.61x | 8.04x | 9.93x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
