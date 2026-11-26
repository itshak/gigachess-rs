| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.29x | 1.85x | 1.56x | 2.42x | 1.90x | 30/30 |
| materialize | 1.00x | 2.23x | 1.70x | 1.70x | 2.63x | 1.91x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.72x | 1.53x | 1.17x | 2.35x | 1.84x | 29/30 |
| perft_full_d3 | 1.23x | 1.65x | 1.96x | 1.85x | 2.41x | 2.04x | 30/30 |
| fen_parse | 0.93x | 1.21x | 1.29x | 1.10x | 1.40x | 1.16x | 30/30 |
| fen_write_native_ep | 0.96x | 4.72x | 9.65x | 1.83x | 26.92x | 2.11x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.33x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.12x | 1.96x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.05x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.08x | 1.31x | 3.91x | 1.31x | 1.65x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.12x | 92.24x | 163.12x | 189.37x | 167.86x | 5/5 |
| chess960_materialize | 1.06x | 1.78x | 1.72x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.55x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 40.75x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 24.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.32x | 2.03x | 1.10x | 2.42x | 2.22x | 9/10 |
| corpus_materialize | 1.00x | 1.39x | 1.29x | 1.07x | 1.69x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.25x | 2.96x | 2.20x | 2.90x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.86x | 1.60x | 1.58x | 2.17x | 2.57x | 9/10 |
| replay_checked_native | 1.01x | 2.01x | 1.07x | 8.67x | 8.29x | 9.80x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
