# Arxiv Server

**Version:** 0.1.0  
**Total Tools:** 5

<details>
<summary><strong>GetPaperById</strong></summary>

**Description:** Fetch a specific paper by its ArXiv ID.

**Parameters:**
- `id`: The ArXiv ID (e.g. '2101.00001v2')

</details>

<details>
<summary><strong>SearchPapers</strong></summary>

**Description:** Search for papers on ArXiv using various criteria.

**Parameters:**
- `query`: The search query string (e.g. 'quantum computing', 'au:"Einstein, Albert"', 'cat:cs.CV')
- `start`: Starting index for results (default: 0)
- `max_results`: Maximum number of results to return (default: 5)
- `sort_by`: Sort field (submittedDate, lastUpdatedDate, relevance)
- `sort_order`: Sort order (ascending, descending)

</details>

<details>
<summary><strong>ListRecords</strong></summary>

**Description:** Bulk harvest metadata from ArXiv using OAI-PMH interface.

**Parameters:**
- `from`: Start date for records (YYYY-MM-DD)
- `until`: End date for records (YYYY-MM-DD)
- `metadata_prefix`: Metadata format (default: oai_dc)
- `set`: Optional set identifier to filter records

</details>

<details>
<summary><strong>SearchByAuthor</strong></summary>

**Description:** Search for papers by a specific author on ArXiv.

**Parameters:**
- `author`: Author name (e.g. 'Einstein, Albert')
- `start`: Starting index for results (default: 0)
- `max_results`: Maximum number of results to return (default: 10)

</details>

<details>
<summary><strong>ExtractPaperText</strong></summary>

**Description:** Extract text from an arXiv paper PDF.

**Parameters:**
- `paper_url`: The arXiv paper URL or ID

</details> 