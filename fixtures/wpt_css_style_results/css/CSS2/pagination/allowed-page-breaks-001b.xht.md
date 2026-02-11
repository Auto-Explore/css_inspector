# css/CSS2/pagination/allowed-page-breaks-001b.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/pagination/allowed-page-breaks-001b.xht"
}
```

## style[0]

```css

	html, body { height: 100%; line-height: 1; font-size: 20px; margin: 0; padding: 0; }
	.spacer { height: 50%; }
	.backup { margin-top: -1em; }
	.start { page-break-before: always; }

	.avoidBefore { page-break-before: avoid; }
	.avoidAfter	{ page-break-after:	avoid; }
	.avoidInside { page-break-inside: avoid; }
	.allowInside { page-break-inside: auto; }

	table, tbody, tr, td {
		border-collapse: collapse;
		margin: 0; padding: 0;
		border-spacing: 0;
		color: blue;
	}
	table, tbody { page-break-inside: avoid; /* override any UA settings */ }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
