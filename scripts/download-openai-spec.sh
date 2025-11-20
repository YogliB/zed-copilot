#!/bin/bash
set -euo pipefail

# Script to download and validate OpenAI API spec
# Downloads the freshest OpenAI OpenAPI spec for use in E2E tests

SPEC_URL="https://app.stainless.com/api/spec/documented/openai/openapi.documented.yml"
SPEC_DIR="tests/schemas"
SPEC_FILE="${SPEC_DIR}/openai.yml"
SPEC_JSON="${SPEC_DIR}/openai.json"
CHECKSUM_FILE="${SPEC_DIR}/.openai-spec.sha256"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Downloading OpenAI API spec...${NC}"

# Create directory if it doesn't exist
mkdir -p "${SPEC_DIR}"

# Download the spec
if ! curl -f -L -o "${SPEC_FILE}" "${SPEC_URL}"; then
    echo -e "${RED}Failed to download OpenAI spec from ${SPEC_URL}${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Downloaded spec to ${SPEC_FILE}${NC}"

# Convert YAML to JSON for easier Rust parsing
if command -v yq &> /dev/null; then
    echo -e "${YELLOW}Converting YAML to JSON...${NC}"
    yq eval -o=json "${SPEC_FILE}" > "${SPEC_JSON}"
    echo -e "${GREEN}✓ Converted to ${SPEC_JSON}${NC}"
else
    echo -e "${YELLOW}Warning: yq not found. Skipping YAML to JSON conversion.${NC}"
    echo -e "${YELLOW}Install with: brew install yq (macOS) or apt-get install yq (Linux)${NC}"
fi

# Calculate and save checksum
if command -v sha256sum &> /dev/null; then
    sha256sum "${SPEC_FILE}" > "${CHECKSUM_FILE}"
elif command -v shasum &> /dev/null; then
    shasum -a 256 "${SPEC_FILE}" > "${CHECKSUM_FILE}"
fi

if [ -f "${CHECKSUM_FILE}" ]; then
    echo -e "${GREEN}✓ Checksum saved to ${CHECKSUM_FILE}${NC}"
    cat "${CHECKSUM_FILE}"
fi

echo -e "${GREEN}✓ OpenAI API spec downloaded successfully${NC}"
echo -e "${YELLOW}Location: ${SPEC_FILE}${NC}"
echo ""
echo "Next steps:"
echo "1. Commit the spec file to git"
echo "2. Run tests: cargo test --test e2e"
echo "3. Re-run this script periodically to update the spec"
