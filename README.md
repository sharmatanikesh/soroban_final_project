# Rental Agreement and Deposit

## Project Name
Rental Agreement and Deposit

## About Me
My name is Tanikesh Sharma, and I'm currently in my 6th semester of pursuing a B.Tech in Computer Science with a specialization in Cybersecurity. I'm passionate about full-stack development and eager to expand my skills. Recently, I've been focusing on learning Web3 technologies. The Soroban Internship Bootcamp has been instrumental in my learning journey, and I'm grateful for the opportunity it provided to explore Web3. Thank you for organizing the bootcamp!

## Project Details
The "Rental Agreement and Deposit" project is a smart contract system designed to streamline and secure the rental agreement process using blockchain technology. By utilizing smart contracts, the project aims to create transparent, immutable, and trustless agreements between tenants and landlords. This system ensures that both parties adhere to the terms of the contract, thereby reducing conflicts and disputes.

## Vision
Our vision is to leverage the power of blockchain technology to create a secure and transparent rental ecosystem. By automating rental agreements and deposit handling through smart contracts, we aim to eliminate the common issues and conflicts that arise in traditional rental arrangements. Our project aspires to set a new standard in the real estate industry by providing a trustworthy and efficient platform for rental transactions.

## Usage
The smart contract provides two main functionalities:

### 1. Create Agreement
This function allows the landlord and tenant to create a rental agreement on the blockchain.
- **Inputs**: Tenant address, landlord address, deposit amount, and the status of the agreement.
- **Process**: The agreement details are stored on the blockchain, ensuring transparency and immutability.

### 2. Get Agreement
This function retrieves the details of an existing rental agreement from the blockchain.
- **Inputs**: Tenant address and landlord address.
- **Output**: The function returns the details of the agreement, including the tenant address, landlord address, deposit amount, and the status of the agreement.

## Benefits
- **Transparency**: All agreement terms are stored on the blockchain, accessible to both parties at any time.
- **Security**: The immutability of blockchain records ensures that the agreement terms cannot be altered once set.
- **Trust**: By removing the need for intermediaries, the smart contract fosters trust between tenants and landlords.
- **Efficiency**: Automating the agreement process saves time and reduces administrative overhead.

## Example Usage
### 1. Create an Agreement
- A landlord and tenant agree on rental terms.
- They call the `create_agreement` function to store these terms on the blockchain.
- The agreement is now securely stored and can be referenced anytime.

### 2. Retrieve an Agreement
- Either party can call the `get_agreement` function using the tenant and landlord addresses.
- The function returns the stored details, allowing both parties to view the agreed-upon terms.
