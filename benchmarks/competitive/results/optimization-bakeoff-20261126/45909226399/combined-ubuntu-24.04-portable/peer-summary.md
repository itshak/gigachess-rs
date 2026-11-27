| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 2.87x | 1.61x | 1.39x | 1.87x | 1.34x | 29/30 |
| materialize | 0.99x | 2.27x | 1.90x | 1.85x | 2.81x | 1.96x | 30/30 |
| perft_bulk_d4 | 0.98x | 2.99x | 1.58x | 1.18x | 1.95x | 1.76x | 29/30 |
| perft_full_d3 | 1.00x | 1.36x | 1.61x | 1.40x | 1.89x | 1.60x | 30/30 |
| fen_parse | 1.03x | 1.51x | 1.43x | 1.19x | 1.52x | 1.20x | 30/30 |
| fen_write_native_ep | 0.99x | 3.00x | 9.75x | 2.02x | 27.33x | 2.51x | 30/30 |
| snapshot | 0.99x | 1.74x | 1.23x | 3.21x | 1.23x | 0.99x | 25/30 |
| san_parse_all | 0.98x | 1.89x | N/A | 4.10x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.28x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 2.08x | 1.41x | 4.08x | 1.32x | 1.50x | 30/30 |
| make_unmake_full | 1.08x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.36x | 61.37x | 108.81x | 124.11x | 98.40x | 5/5 |
| chess960_materialize | 0.94x | 1.73x | 1.66x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.15x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 37.84x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.18x | 23.22x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 4.12x | 2.04x | 1.12x | 2.09x | 2.80x | 10/10 |
| corpus_materialize | 0.99x | 1.51x | 1.41x | 1.13x | 1.83x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.95x | 3.01x | 2.24x | 2.93x | 1.62x | 10/10 |
| replay_trusted_native | 0.92x | 0.83x | 1.60x | 1.53x | 2.09x | 1.88x | 9/10 |
| replay_checked_native | 1.01x | 1.99x | 1.16x | 8.70x | 8.39x | 9.89x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
