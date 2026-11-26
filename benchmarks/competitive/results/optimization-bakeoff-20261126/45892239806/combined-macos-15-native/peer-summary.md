| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.22x | 4.72x | 1.53x | 1.69x | 2.41x | 2.02x | 30/30 |
| materialize | 1.09x | 2.06x | 1.60x | 1.07x | 2.19x | 1.34x | 28/30 |
| perft_bulk_d4 | 1.04x | 3.99x | 1.51x | 1.16x | 1.93x | 1.84x | 30/30 |
| perft_full_d3 | 0.76x | 1.19x | 1.19x | 0.96x | 1.36x | 1.41x | 27/30 |
| fen_parse | 1.14x | 1.24x | 1.36x | 1.40x | 1.67x | 1.51x | 30/30 |
| fen_write_native_ep | 1.04x | 3.93x | 8.41x | 1.66x | 26.86x | 2.03x | 30/30 |
| snapshot | 1.13x | 1.88x | 1.19x | 2.49x | 1.17x | 1.09x | 26/30 |
| san_parse_all | 1.13x | 1.88x | N/A | 4.07x | N/A | N/A | 12/12 |
| san_render_native | 1.08x | 12.48x | N/A | 2.81x | N/A | N/A | 12/12 |
| apply_children_full | 1.12x | 1.17x | 1.27x | 6.12x | 1.27x | 1.39x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.11x | N/A | N/A | 4/6 |
| startpos | 2.01x | 2.41x | 158.36x | 285.41x | 399.39x | 276.01x | 5/5 |
| chess960_materialize | 1.13x | 1.69x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.85x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.16x | 40.57x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.11x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.22x | 21.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.49x | 1.86x | 1.09x | 2.11x | 2.30x | 9/10 |
| corpus_materialize | 1.17x | 1.46x | 1.39x | 1.13x | 1.98x | 1.23x | 10/10 |
| corpus_uci_sorted | 1.11x | 1.94x | 2.79x | 1.97x | 2.49x | 1.51x | 10/10 |
| replay_trusted_native | 1.05x | 0.61x | 1.55x | 1.54x | 1.69x | 1.67x | 8/10 |
| replay_checked_native | 1.11x | 2.54x | 1.18x | 9.28x | 8.08x | 10.21x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
