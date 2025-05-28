import React, { useState, useEffect } from 'react';
import './App.css';
import ApiService from './services/api';

function App() {
  const [users, setUsers] = useState([]);
  const [newUserName, setNewUserName] = useState('');
  const [loading, setLoading] = useState(false);

  // Fetch users on component mount
  useEffect(() => {
    fetchUsers();
  }, []);

  const fetchUsers = async () => {
    try {
      setLoading(true);
      const usersData = await ApiService.getAllUsers();
      setUsers(usersData);
    } catch (error) {
      console.error('Error fetching users:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateUser = async (e) => {
    e.preventDefault();
    if (!newUserName.trim()) return;

    try {
      setLoading(true);
      await ApiService.createUser({ name: newUserName });
      setNewUserName('');
      await fetchUsers(); // Refresh the list
    } catch (error) {
      console.error('Error creating user:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteUser = async (id) => {
    try {
      setLoading(true);
      await ApiService.deleteUser(id);
      await fetchUsers(); // Refresh the list
    } catch (error) {
      console.error('Error deleting user:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>Secure Image Sharing System</h1>
        
        {/* Create User Form */}
        <form onSubmit={handleCreateUser}>
          <input
            type="text"
            value={newUserName}
            onChange={(e) => setNewUserName(e.target.value)}
            placeholder="Enter user name"
            disabled={loading}
          />
          <button type="submit" disabled={loading || !newUserName.trim()}>
            Add User
          </button>
        </form>

        {/* Users List */}
        <div>
          <h2>Users</h2>
          {loading ? (
            <p>Loading...</p>
          ) : (
            <ul>
              {users.map(user => (
                <li key={user.id}>
                  {user.name} (ID: {user.id})
                  <button onClick={() => handleDeleteUser(user.id)}>
                    Delete
                  </button>
                </li>
              ))}
            </ul>
          )}
        </div>
      </header>
    </div>
  );
}

export default App;