export interface Vault {
  db_id?: number;
  uid: string;
  user_id: number;
  name: string;
  created_at: number;
  updated_at: number;
}

export interface VaultResult {
  success?: Vault;
  error?: string;
}
